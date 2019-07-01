## ANDROID

### Enable USB debugging in the Developer options as follows.

First, you must enable the developer options:

-   Open the Settings app.
-   (Only on Android 8.0 or higher) Select System.
-   Scroll to the bottom and select About phone.
-   Scroll to the bottom and tap Build number 7 times.
-   Return to the previous screen to find Developer options near the bottom.

Open Developer options, and then scroll down to find and enable USB debugging.

### See ip address

- Open the Settings app.
- Scroll to the bottom and select About phone.
- Tap Status

### Enable LAN debugging

```bash
adb usb
adb tcpip 5555
adb connect 192.168.1.108:5555
adb devices
adb kill-server
```

### Documents

-   [Andorid SDK](https://wiki.archlinux.org/index.php/android)
-   [Configuring VM acceleration on Linux](https://developer.android.com/studio/run/emulator-acceleration?utm_source=android-studio#vm-linux)
-   [Android Material Design Icon Generator Plugin](https://github.com/konifar/android-material-design-icon-generator-plugin)
-   [Distribution dashboard](https://developer.android.com/about/dashboards/index.html)

```bash
egrep -c '(vmx|svm)' /proc/cpuinfo
```
