# TOTP-CLI

Minimalist TUI application that displays TOTP.

## Note

This is not cross-platform, only tested on Linux and might work on other Unix-like systems, but never tested.

## Key-bindings

- `k` to move up.
- `j` to move down.
- `q` to quit the application.
- `Enter` to copy the TOTP from the current line to the clipboard.

## Configuration file

It uses the URI string format, the same format used by Google authenticator and many other OTP applications.

Example of configuration.

```
// This is a comment
Otpauth://totp/MyKeyName:me@my-domain.com?secret=GZMWV5JLOMNI2XJL&issuer=AcmeCorp
Otpauth://totp/MyKeyName:me@my-domain.com?secret=JXQWZ4TVRNUP5YKM&issuer=WidgetCo
Otpauth://totp/MyKeyName:me@my-domain.com?secret=KBYXA6USSPQ7ZLNN&issuer=FoobarInc
Otpauth://totp/MyKeyName:me@my-domain.com?secret=LCZYB7VTTSR8AMOO&issuer=GlobexCorp
Otpauth://totp/MyKeyName:me@my-domain.com?secret=MDAZC8WUUTS9BNPP&issuer=BigCorp

// This is another comment
Otpauth://totp/MyKeyName:me@my-domain.com?secret=NEBAD9XVVUT0COQQ&issuer=SmallFirm
Otpauth://totp/MyKeyName:me@my-domain.com?secret=OFCAE0YWWVU1DPRR&issuer=MegaCorp
Otpauth://totp/MyKeyName:me@my-domain.com?secret=PGDBF1ZXWXU2EQSS&issuer=TechCo
Otpauth://totp/MyKeyName:me@my-domain.com?secret=QHECK2AYXYU3FRTT&issuer=StartupInc
Otpauth://totp/MyKeyName:me@my-domain.com?secret=RIFDL3BZYZU4GSUU&issuer=ConsultingFirm
```

By default the secrets file should be located at `$HOME/.config/totp-cli/totp.txt` the name of the file can be switched out parsing in another filename, this is useful if multiple setups are needed.

```sh
totp-cli "my_other_totp_secrets_file.txt"
```
