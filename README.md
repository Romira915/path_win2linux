# path_win2linux

Converts a path from Windows notation to unix notation

## Usage

```sh
./path_win2linux -d <dir> -e <extension>
```

```sh
./path_win2linux -f <file>
```

Convert m3u file in current directory:
```sh
./path_win2linux
```

#### Notes column
行っている処理は'\'を'/'に変換しているだけです．
パスかどうかの判定は行っていないので，ファイル内の全てを置き換えます．ご利用の際はご注意ください．