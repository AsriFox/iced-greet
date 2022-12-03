# iced-greet
Graphical greeter for *[greetd](https://git.sr.ht/~kennylevinsen/greetd)* made with *[iced](https://github.com/iced-rs/iced)*

## Features
* Input username directly or select from the list (queries */etc/passwd*)
* Launch [any program](https://man.sr.ht/~kennylevinsen/greetd/#what-can-greetd-start) on authentification
* Shutdown (SD) or restart (RE)

## Planned
* Configuration/theming
* Select commands from a predefined list
* * Session files (*/usr/share/xsessions* and */usr/share/wayland-sessions*)
* * Custom commands from the config file
* User avatar in a circle

## Install
1. [Install greetd](https://git.sr.ht/~kennylevinsen/greetd#installation)
2. Build *iced-greet* (e.g. `cargo build --release`)
3. Copy the executable to the global PATH location (e.g. `sudo cp target/release/iced-greet /usr/local/bin/`)
4. Install a Wayland compositor, such as *cage* or *sway*
5. Edit */etc/greetd/config.toml* to set the new greeter:
```
...
[default_session]
command = "cage -s -- iced-greet"
user = "greeter"
```
Read [greetd manual](https://man.sr.ht/~kennylevinsen/greetd/#setting-up-greetd) for more clarification.

## Troubleshooting

### Graphics drivers
*iced-greet* is confirmed to work in *cage* on an NVIDIA card with **[nouveau](https://wiki.archlinux.org/title/nouveau)** drivers. Proprietary **nvidia** drivers won't work because *cage*/*sway* themselves don't work with them. I have not yet tried any other Wayland compositors nor AMD/Intel devices.

### Launch fails
If you see error messages and/or the greeter does not launch, you may be locked out of your system. Try switching to another VT to log in through *agetty* or something and edit */etc/greetd/config.toml* to use the default *agreety* or any other greeter.

If VT switching fails, *greetd* manual suggests [modifying kernel command line in your bootloader](https://man.sr.ht/~kennylevinsen/greetd/#i-used-cage-as-my-greeter-messed-up-my-config-and-now-i-canx27t-log-in-or-switch-to-another-vt) to disable *greetd.service*, however, I couldn't do it. Instead, I use a LiveUSB with a distro installer to `mount` the system partition, `chroot` into it and edit the config.

## Development

### Testing
To debug/run the application without connecting to *greetd* socket (which will fail if the user is not in *greeter* group), use 'test' command line argument (e.g. `iced-greet test` or `cargo run -- test`). Note that this is **not** the same application as the main one, so you will need to copy the desired changes to the *ui* module before building to install.

### Backend
The application uses *iced*'s *glow* (OpenGL) backend. With *wgpu* backend the window loads for a very long time. *perf* results suggest that it has something to do with Vulkan driver allocation for NVIDIA. I plan to test *wgpu* with an AMD card in the future.
