# remember-to-stand

A simple app run from the command line to remind you to stand up every now and then while working.

I'm not a Rust expert but have been intrigued with the language, and hence decided to write a simple utility to remind myself to stand up every now and then at my desk. If you don't have a standing desk, you might want to use this to remind you to stand and take a walk around every now and then to stretch your legs. 

Remember to stand has a customisable time delay between the two different notification modes. It will pop up 'toast' notifications with a sound effect on each notification between standing and sitting mode.

When run, it will add a small icon in the system tray area (tested on macOS and uBuntu Linux 22.04).

## Build

To build you'll need to have the Rust tool chain with cargo installed. Compile into a single executable file with:

- `./make.sh build`

## Run

Once built, you can run the executable directly - e.g. `./target/debug/remember-to-stand`, or run directly from source with:

- `cargo run`

## Install

To install, build the app in release mode with:

- `./make.sh build --release`

Then you can place the release executable in a convenient location. (For now you'll need to copy the system tray icon and sound file to the same location too - copy from the **./resources** or the target build directory to the same path that you place the executable in).

## Configure

The app will automatically create a default configuration file in your user home path: **~/.remembertostand**. You can edit this file to change the notification title text messages and customise the time delays between sitting and standing modes.

For example:

```json
{
    "config": {
        "customstandmsg": "Stand up",
        "standtimesecs": "3600",
        "sittimesecs": "3600",
        "customsitmsg": "Sit down"
    }
}
```
