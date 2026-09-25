-- Integrates the `recol` CLI with Neovim.
-- Commands: :Recol [args] (-i/--interactive opens a floating terminal),
--           :RecolOpen (shortcut for interactive mode).
-- Config is reloaded automatically after recol runs.
if vim.fn.executable("recol") == 1 then
    local function reload_config()
        local ok, err = pcall(vim.cmd.source, vim.fn.stdpath("config") .. "/init.lua")
        if not ok then
            vim.notify("Recol: config reload failed: " .. tostring(err), vim.log.levels.ERROR)
        end
    end
    local function launch_interactive_mode()
        local width = math.floor(vim.o.columns * 0.75)
        local height = math.floor(vim.o.lines * 0.75)
        local buf = vim.api.nvim_create_buf(false, true)
        local win = vim.api.nvim_open_win(buf, true, {
            relative = "editor",
            width = width,
            height = height,
            row = math.floor((vim.o.lines - height - 3) / 2),
            col = math.floor((vim.o.columns - width) / 2),
            border = "rounded",
            title = " Recol ",
            title_pos = "center",
        })
        vim.bo[buf].bufhidden = "wipe"
        local job_id = vim.fn.termopen({ "recol", "-i", "--quit-on-select" }, {
            on_exit = function(_, code)
                vim.schedule(function()
                    if vim.api.nvim_win_is_valid(win) then
                        vim.api.nvim_win_close(win, true)
                    end
                    if code == 0 then
                        reload_config()
                    end
                end)
            end,
        })
        if job_id <= 0 then
            vim.notify("Recol: failed to start process", vim.log.levels.ERROR)
            vim.api.nvim_win_close(win, true)
            return
        end
        vim.cmd.startinsert()
    end
    vim.api.nvim_create_user_command("Recol", function(opts)
        local args = vim.split(opts.args, "%s+", { trimempty = true })
        if vim.tbl_contains(args, "-i") or vim.tbl_contains(args, "--interactive") then
            return launch_interactive_mode()
        end
        local cmd = vim.list_extend({ "recol" }, args)
        vim.system(cmd, { text = true }, function(res)
            vim.schedule(function()
                if res.code ~= 0 then
                    vim.notify("Recol: exit " .. res.code .. "\n" .. (res.stderr or ""), vim.log.levels.ERROR)
                end
                reload_config()
            end)
        end)
    end, { nargs = "*", desc = "Run recol (-i for interactive mode)" })
    vim.api.nvim_create_user_command("RecolOpen", launch_interactive_mode,
        { nargs = 0, desc = "Open recol interactively" })
end
