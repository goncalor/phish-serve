FROM archlinux
RUN pacman --noconfirm -Syu

RUN pacman --noconfirm -S python python-pip
RUN pip install --upgrade https://github.com/goncalor/python-docx/tarball/master

WORKDIR /app
RUN useradd -d /app user

USER user

COPY ["docm-morph.py", "phish-serve", "phish-serve-dev", "Rocket.toml", "./"]
