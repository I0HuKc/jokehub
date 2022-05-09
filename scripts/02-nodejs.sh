!/bin/bash

Скрипт автоматизированной утановки NodeJs

curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

source ~/.bashrc

# TODO: Проверять существует ли файл
source ~/.bash_profile

nvm list-remote

# TODO: запрашивать ввод нужной версии

nvm install v16.14.2