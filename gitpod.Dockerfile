FROM gitpod/workspace-full-vnc
RUN sudo apt-get update && \
    sudo apt-get install -y libgtk-3-dev && \
    sudo rm -rf /var/lib/apt/lists/*

RUN curl -o flutter.tar.xz https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.16.8-stable.tar.xz \
  && mkdir -p /opt/flutter \
  && tar xf flutter.tar.xz -C /opt \
  && rm flutter.tar.xz \
  && flutter config --no-analytics \
  && flutter precache 

RUN echo 'PATH="$PATH:/opt/flutter/bin' >> /etc/profile
