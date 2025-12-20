# tvibe

tvibe — простая утилита для изменения и синхронизации цветовой темы Neovim и Alacritty.

Программа содержит более 50 готовых цветовых схем и автоматически изменяет конфигурационные файлы для смены темы сразу в обоих приложениях.

## Возможности

- Синхронная смена темы для nvim и alacritty
- Более 50 встроенных цветовых схем
- Случайный выбор темы
- Случайный выбор светлой или тёмной темы
- Поиск и применение темы по названию
- Просмотр списка доступных тем
- Изменить Nerd шрифт в alacritty
- Просмотр списка установденных Nerd шрифтов
- Умный поиск при установки темы или шрифта (не точный ввод)

## Использование

```text
theme changing utility

Usage: tvibe [OPTIONS]

Options:
  -r, --rand           Set random theme
  -l, --light-rand     Set random light theme
  -d, --dark-rand      Set random dark theme
  -q, --query <QUERY>  Search and apply theme
      --list           List of available themes
  -f, --font <FONT>    Change alacritty nerd font family
      --font-list      List of nerd font family
  -h, --help           Print help
  -V, --version        Print version
```

tvibe содержит встроенный набор готовых тем.

Настройки тем для Neovim основаны на проекте  
https://github.com/EdenEast/nightfox.nvim  
и адаптированы под другие цветовые схемы.

Большинство цветовых схем взяты отсюда, там же можно посмотреть внешний вид темы:  
https://github.com/alacritty/alacritty-theme

При выборе темы программа автоматически правит конфигурацию:

- Neovim
- Alacritty

## Список доступных тем

```text
ashes_dark
ashes_light
autumn
base16_dark
chicago95
dayfox
duskfox
github_dark
github_dark_tritanopia
github_light
google
gotham
gruber_darker
gruvbox_dark
gruvbox_light
gruvbox_material_hard_dark
gruvbox_material_hard_light
gruvbox_material_medium_dark
gruvbox_material_medium_light
hardhacker
high_contrast
horizon_dark
hyper
iceberg
iris
iterm2
kanagawa_dragon
kanagawa_wave
kimbie_dark
kimbie_light
kitty
konsole_port
low_contrast
marine_dark
meliora
miasma
midnight_haze
monokai
monokai_charcoal
monokai_pro
moonfly
neobones_dark
neobones_light
night_owl
nightfox
nordfox
paper
rose_pine
rose_pine_dawn
terafox
tokyo_night
ubuntu
vesper
vscode_dark_plus
xcode_dark
xcode_light
```
