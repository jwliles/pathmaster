export ZSH="$HOME/.oh-my-zsh"

# Initialize the path array with unique entries
typeset -U path

# Append directories to the path array
path+=(
  "$HOME/Applications"
  "$HOME/Applications/bin"
  "$HOME/Applications/scripts"
  "$HOME/.local/bin"
  "$HOME/.emacs.d/bin"
  "$HOME/.config/emacs/bin"
  "/usr/local/texlive/2023/bin/x86_64-linux"
  "$HOME/bin"
  "$HOME/.local/share/gem/ruby/3.3.0/bin"
  "/snap/bin"
  "/usr/local/go/bin"
  "$HOME/perl5/bin"
  "$HOME/.rvm/bin"
)

# Ensure unique path entries
typeset -U path

# Export PATH from path array
export PATH

# Set oh-my-zsh update mode
zstyle ':omz:update' mode auto # update automatically without asking

# Enable command auto-correction
ENABLE_CORRECTION="true"

# Disable marking untracked files under VCS as dirty
DISABLE_UNTRACKED_FILES_DIRTY="true"

# Set history timestamp format
HIST_STAMPS="YYYY-MM-DD"

# Plugins to load
plugins=(
  git
  zsh-autosuggestions
  zsh-syntax-highlighting
  zsh-interactive-cd
  history
  sudo
  zoxide
)

# Source oh-my-zsh
source $ZSH/oh-my-zsh.sh

# Initialize oh-my-posh after sourcing oh-my-zsh
eval "$(oh-my-posh init zsh --config $HOME/.config/ohmyposh/naut.toml)"

# User configuration

# Set preferred editor
export EDITOR='vim'

# Set bat config path
export BAT_CONFIG_PATH="$HOME/.config/bat/config.conf"

# Define aliases
alias snapls='sudo snapper list'
alias matrix='cmatrix -s -C cyan'
alias iso="cat /etc/dev-rel | awk -F '=' '/ISO/ {print \$2}'"
alias probe='sudo -E hw-probe -all -upload'

# Replace ls with exa
alias ls='exa -al --color=always --group-directories-first --icons' # preferred listing
alias la='exa -a --color=always --group-directories-first --icons'  # all files and dirs
alias ll='exa -l --color=always --group-directories-first --icons'  # long format
alias lt='exa -aT --color=always --group-directories-first --icons' # tree listing
alias l='exa -lah --color=always --group-directories-first --icons' # tree listing

# Pacman aliases
alias unlock='sudo rm /var/lib/pacman/db.lck'
alias free='free -mt'
alias wget='wget -c'
alias df='df -h'
alias userlist='cut -d: -f1 /etc/passwd'
alias upall='topgrade'
alias search='sudo pacman -Qs'
alias remove='sudo pacman -R'
alias install='sudo pacman -S'
alias linstall='sudo pacman -U'
alias pupdate='sudo pacman -Syyu'
alias clrcache='sudo pacman -Scc'
alias orphans='sudo pacman -Rns $(pacman -Qtdq)'
alias akring='sudo pacman -Sy archlinux-keyring --noconfirm'

# Paru/Yay aliases
alias pget='paru -S'
alias yget='yay -Syyu'
alias yrem='yay -R'
alias prem='paru -R'
alias look='yay -Ss'

# Flatpak Update
alias fpup='flatpak update'

# Skip integrity check
alias paruskip='paru -S --mflags --skipinteg'
alias yayskip='yay -S --mflags --skipinteg'

# Grub update
alias grubup='sudo grub-mkconfig -o /boot/grub/grub.cfg'

# Reflector mirrorlist updates
alias mirrorx="sudo reflector --age 6 --latest 20  --fastest 20 --threads 5 --sort rate --protocol https --save /etc/pacman.d/mirrorlist"
alias mirrorxx="sudo reflector --age 6 --latest 20  --fastest 20 --threads 20 --sort rate --protocol https --save /etc/pacman.d/mirrorlist"

# General aliases
alias mkfile='touch'
alias jctl='journalctl -p 3 -xb'
alias breload='cd ~ && source ~/.bashrc'
alias zreload='cd ~ && source ~/.zshrc'
alias pingme='ping -c64 github.com'
alias cls='clear && neofetch'
alias traceme='traceroute github.com'
alias hw='hwinfo --short'

# YouTube-dl aliases
alias yta-best="yt-dlp --extract-audio --audio-format best"
alias ytv-best="yt-dlp -f 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio' --merge-output-format mp4"

# Git aliases
alias gc='git clone'
alias gp='git pull'
alias cgc='cfg commit -s -m "Update dotfiles"'
alias cpp='cfg push && cfg push --tags'
alias css='cfg status'
alias caa='cfg add -u'
alias gcl='git fsck --full && git reflog expire --expire=now --all && git repack -a -d -l && git gc --prune=now --aggressive && git lfs fetch --prune'
alias gfppt='git lfs fetch && git lfs pull && git pull && git push && git push --tags'

# Copy/Remove files/dirs
alias rmd='rm -r'
alias srm='sudo rm'
alias srmd='sudo rm -r'
alias cpd='cp -R'
alias scp='sudo cp'
alias scpd='sudo cp -R'

# Directory navigation aliases
alias home='cd ~'
alias etc='cd /etc/'
alias music='cd ~/Music'
alias vids='cd ~/Videos'
alias conf='cd ~/.config'
alias desk='cd ~/Desktop'
alias pics='cd ~/Pictures'
alias dldz='cd ~/Downloads'
alias docs='cd ~/Documents'
alias sapps='cd /usr/share/applications'
alias lapps='cd ~/.local/share/applications'

# GPG key retrieval
alias gpg-retrieve='gpg2 --keyserver-options auto-key-retrieve --receive-keys'

# Switch between lightdm and sddm
alias tolightdm="sudo pacman -S lightdm lightdm-gtk-greeter lightdm-gtk-greeter-settings --noconfirm --needed; sudo systemctl enable lightdm.service -f; echo 'Lightdm is active - reboot now'"
alias tosddm="sudo pacman -S sddm --noconfirm --needed; sudo systemctl enable sddm.service -f; echo 'Sddm is active - reboot now'"

# Recent Installed Packages
alias rip="expac --timefmt='%Y-%m-%d %T' '%l\t%n %v' | sort | tail -200 | nl"
alias riplong="expac --timefmt='%Y-%m-%d %T' '%l\t%n %v' | sort | tail -3000 | nl"

# Package Info
alias info='sudo pacman -Si'
alias infox='sudo pacman -Sii'

# Refresh Keys
alias rkeys='sudo pacman-key --refresh-keys'

# Shutdown or reboot
alias sr='sudo reboot'
alias ssn='sudo shutdown now'

# Vim commands
alias v.='vim ~/.vim/vimrc'
alias nv='nvim'

# BFG Repo-Cleaner
alias bfg='java -jar bfg-1.13.0.jar'

# History file location
export HISTFILE=~/.zsh_history
export HISTFILESIZE=1000000000
export HISTSIZE=1000000000

# Ignore duplicates when saving history
setopt HIST_IGNORE_ALL_DUPS

# GPG pinentry
export GPG_TTY=$(tty)
gpg-connect-agent updatestartuptty /bye >/dev/null

# Perl environment variables
export PERL5LIB="$HOME/perl5/lib/perl5${PERL5LIB:+:${PERL5LIB}}"
export PERL_LOCAL_LIB_ROOT="$HOME/perl5${PERL_LOCAL_LIB_ROOT:+:${PERL_LOCAL_LIB_ROOT}}"
export PERL_MB_OPT="--install_base \"$HOME/perl5\""
export PERL_MM_OPT="INSTALL_BASE=$HOME/perl5"

# Alias for luamake
alias luamake="$HOME/Applications/lua-language-server/3rd/luamake/luamake"

# Initialize zoxide
eval "$(zoxide init --cmd cd zsh)"

# Initialize atuin
. "$HOME/.atuin/bin/env"
eval "$(atuin init zsh)"

# FZF configuration
[ -f /usr/share/fzf/key-bindings.zsh ] && source /usr/share/fzf/key-bindings.zsh

# Use fzf for history search
fzf-history-widget() {
  local selected
  selected=$(fc -l 1 | fzf --height 40% --reverse --tiebreak=index --bind=ctrl-r:toggle-sort+down+up+toggle-sort | sed 's/^[ 0-9]*//')
  LBUFFER+=$selected
}
zle -N fzf-history-widget
bindkey '^r' fzf-history-widget

# Correct the trap command
trap "$HOME/.dotfiles/scripts/.scripts/backup_history.sh" EXIT
export TSTRUCT_TOKEN="tstruct_eyJ2ZXJzaW9uIjoxLCJkYXRhIjp7InVzZXJJRCI6NzYxMTEwOTEzLCJ1c2VyRW1haWwiOiJqd2xpbGVzQG91dGxvb2suY29tIiwidGVhbUlEIjoyMDk0MzcxOTYxLCJ0ZWFtTmFtZSI6Imp3bGlsZXNAb3V0bG9vay5jb20ncyB0ZWFtIiwicmVuZXdhbERhdGUiOiIyMDI1LTA1LTI5VDE1OjAwOjQyWiIsImNyZWF0ZWRBdCI6IjIwMjQtMDktMDlUMTg6MjE6NTkuMTI1NzI3MzU2WiJ9LCJzaWduYXR1cmUiOiJ3cTU2dUxRSHFhcWVhOTM2OFB0c1QyVllyeUwvQ0lJMmo3Q1RkZzBWQVl5ZEJ1dVBSNHUxZjI3NUlycTc3U1k2TzdabXprR1lRd0hScFJrSkxraDlEUT09In0="

# >>> juliaup initialize >>>

# !! Contents within this block are managed by juliaup !!

path=('/home/jwl/.juliaup/bin' $path)
export PATH

# <<< juliaup initialize <<<

# Fetchos system information display
#if [ -x "$HOME/Applications/fetchos" ]; then
#  $HOME/Applications/fetchos
#fi

# Add RVM to PATH for scripting. Make sure this is the last PATH variable change.
export XDG_SESSION_TYPE=wayland
export XMODIFIERS=@im=espanso
export GTK_IM_MODULE=xim
export QT_IM_MODULE=xim
export BROWSER=vivaldi-stable
