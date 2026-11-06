_recol() {
  local IFS=$'\n'
  COMPREPLY=($(recol --complete "$COMP_CWORD" "${COMP_WORDS[@]}" 2>/dev/null))
}
[ -n "$ZSH_VERSION" ] && autoload -Uz bashcompinit && bashcompinit
complete -o default -F _recol recol
