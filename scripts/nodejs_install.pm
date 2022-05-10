#!/usr/bin/perl

use Env;
use Env qw(PATH HOME TERM);
use Env qw($SHELL @LD_LIBRARY_PATH);
use Term::ANSIColor qw(:constants);

sub check_dir($$) {
    my ($dir, $in) = @_;

    if (-d "$in/$dir") {
        return 0;
    }

    return 1;
}

sub check_file($$) {
    my ($file, $in) = @_;

    if (-e "$in/$file") {
        return 0;
    }

    return 1;
}

sub main() {
    # Проверяю установлен ли nvm
    if (check_dir(".nvdm", $HOME) != 0) {
        print YELLOW, "NVM is not installed\n", RESET;
        print "Installing NVM...\n", RESET;

        # Качаю nvm
        system("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash");

        # Пpоверяю скачалось ли
        if (check_dir(".nvm", $HOME) != 0) {
            print YELLOW, "Can't install package NVM\n", RESET;
            exit 1;
        }

        # Установка пакета
        my $nvm_dir= "$HOME/.nvm";
        system(`[ -s "$nvm_dir/nvm.sh" ] && \. "$nvm_dir/nvm.sh"`)
        system(`[ -s "$nvm_dir/bash_completion" ] && \. "$nvm_dir/bash_completion"`)

        if (check_file(".bashrc", $HOME) != 1) {
            system("source ~/.bashrc");
        }

        if (check_file(".bash_profile", $HOME) != 1) {
            system("source ~/.bash_profile");
        }

        # Установка нужной версии 
        print "Install version: "; 
        my $version = <>;

        system("nvm install $version")
    }

    print GREEN, "NodeJs is installed\n", RESET;
    exit
}


main()