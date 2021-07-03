#compdef menyoki

autoload -U is-at-least

_menyoki() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-c+[Set the configuration file]' \
'--config=[Set the configuration file]' \
'--color=[Set the main color]' \
'*-v[Increase logging verbosity]' \
'*--verbose[Increase logging verbosity]' \
'-q[Do not show output]' \
'--quiet[Do not show output]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
":: :_menyoki_commands" \
"*::: :->menyoki" \
&& ret=0
    case $state in
    (menyoki)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-command-$line[1]:"
        case $line[1] in
            (record)
_arguments "${_arguments_options[@]}" \
'--action-keys=[Set the action keys]' \
'--cancel-keys=[Set the cancel keys]' \
'-b+[Set the border width]' \
'--border=[Set the border width]' \
'-p+[Set the record area padding]' \
'--padding=[Set the record area padding]' \
'-s+[Set the record area size]' \
'--size=[Set the record area size]' \
'-d+[Set the duration for recording]' \
'--duration=[Set the duration for recording]' \
'-c+[Set the countdown before recording]' \
'--countdown=[Set the countdown before recording]' \
'-t+[Set the timeout for window selection]' \
'--timeout=[Set the timeout for window selection]' \
'-i+[Set the refresh interval for window selection]' \
'--interval=[Set the refresh interval for window selection]' \
'--font=[Set the font to use for window selection]' \
'--monitor=[Set the monitor to record as root window]' \
'-r[Record the root window]' \
'--root[Record the root window]' \
'(-r --root)-f[Record the focused window]' \
'(-r --root)--focus[Record the focused window]' \
'--select[Select the window to record]' \
'--parent[Record the parent of the window]' \
'--with-alpha[Record with the alpha channel]' \
'--no-keys[Disable the action keys while recording]' \
'-m[Select the window with mouse click]' \
'--mouse[Select the window with mouse click]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::command -- Set the command to run:_files' \
":: :_menyoki__record_commands" \
"*::: :->record" \
&& ret=0
case $state in
    (record)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-record-command-$line[2]:"
        case $line[2] in
            (gif)
_arguments "${_arguments_options[@]}" \
'-f+[Set the FPS]' \
'--fps=[Set the FPS]' \
'-q+[Set the frame quality (1-100)]' \
'--quality=[Set the frame quality (1-100)]' \
'-r+[Set the number of repetitions]' \
'--repeat=[Set the number of repetitions]' \
'-s+[Set the animation speed]' \
'--speed=[Set the animation speed]' \
'--cut-beginning=[Cut the beginning of the animation]' \
'--cut-end=[Cut the end of the animation]' \
'-d+[Set the directory to read frames]' \
'--dir=[Set the directory to read frames]' \
'--format=[Set the animation format]: :(gif apng)' \
'--gifski[Use the gifski encoder]' \
'--fast[Encode 3 times faster (gifski)]' \
'-n[Use frames in the order given]' \
'--no-sort[Use frames in the order given]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::frames -- Set the animation frames:_files' \
":: :_menyoki__record__gif_commands" \
"*::: :->gif" \
&& ret=0
case $state in
    (gif)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-record-gif-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(apng)
_arguments "${_arguments_options[@]}" \
'-f+[Set the FPS]' \
'--fps=[Set the FPS]' \
'-q+[Set the frame quality (1-100)]' \
'--quality=[Set the frame quality (1-100)]' \
'-r+[Set the number of repetitions]' \
'--repeat=[Set the number of repetitions]' \
'-s+[Set the animation speed]' \
'--speed=[Set the animation speed]' \
'--cut-beginning=[Cut the beginning of the animation]' \
'--cut-end=[Cut the end of the animation]' \
'-d+[Set the directory to read frames]' \
'--dir=[Set the directory to read frames]' \
'--format=[Set the animation format]: :(gif apng)' \
'--gifski[Use the gifski encoder]' \
'--fast[Encode 3 times faster (gifski)]' \
'-n[Use frames in the order given]' \
'--no-sort[Use frames in the order given]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::frames -- Set the animation frames:_files' \
":: :_menyoki__record__apng_commands" \
"*::: :->apng" \
&& ret=0
case $state in
    (apng)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-record-apng-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(extract)
_arguments "${_arguments_options[@]}" \
'-d+[Set the output directory]' \
'--dir=[Set the output directory]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':file -- Set the animation file:_files' \
":: :_menyoki__split_commands" \
"*::: :->split" \
&& ret=0
case $state in
    (split)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-command-$line[2]:"
        case $line[2] in
            (png)
_arguments "${_arguments_options[@]}" \
'-c+[Set the compression level]: :(default fast best huffman rle)' \
'--compression=[Set the compression level]: :(default fast best huffman rle)' \
'-f+[Set the filter algorithm]: :(none sub up avg paeth)' \
'--filter=[Set the filter algorithm]: :(none sub up avg paeth)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__png_commands" \
"*::: :->png" \
&& ret=0
case $state in
    (png)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-png-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(jpg)
_arguments "${_arguments_options[@]}" \
'-q+[Set the image quality (1-100)]' \
'--quality=[Set the image quality (1-100)]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__jpg_commands" \
"*::: :->jpg" \
&& ret=0
case $state in
    (jpg)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-jpg-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(bmp)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__bmp_commands" \
"*::: :->bmp" \
&& ret=0
case $state in
    (bmp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-bmp-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ico)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__ico_commands" \
"*::: :->ico" \
&& ret=0
case $state in
    (ico)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-ico-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tiff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__tiff_commands" \
"*::: :->tiff" \
&& ret=0
case $state in
    (tiff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-tiff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tga)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__tga_commands" \
"*::: :->tga" \
&& ret=0
case $state in
    (tga)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-tga-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(pnm)
_arguments "${_arguments_options[@]}" \
'-f+[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'--format=[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'-e+[Set the encoding for storing the samples]: :(binary ascii)' \
'--encoding=[Set the encoding for storing the samples]: :(binary ascii)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__pnm_commands" \
"*::: :->pnm" \
&& ret=0
case $state in
    (pnm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-pnm-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__ff_commands" \
"*::: :->ff" \
&& ret=0
case $state in
    (ff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-ff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(split)
_arguments "${_arguments_options[@]}" \
'-d+[Set the output directory]' \
'--dir=[Set the output directory]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':file -- Set the animation file:_files' \
":: :_menyoki__split_commands" \
"*::: :->split" \
&& ret=0
case $state in
    (split)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-command-$line[2]:"
        case $line[2] in
            (png)
_arguments "${_arguments_options[@]}" \
'-c+[Set the compression level]: :(default fast best huffman rle)' \
'--compression=[Set the compression level]: :(default fast best huffman rle)' \
'-f+[Set the filter algorithm]: :(none sub up avg paeth)' \
'--filter=[Set the filter algorithm]: :(none sub up avg paeth)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__png_commands" \
"*::: :->png" \
&& ret=0
case $state in
    (png)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-png-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(jpg)
_arguments "${_arguments_options[@]}" \
'-q+[Set the image quality (1-100)]' \
'--quality=[Set the image quality (1-100)]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__jpg_commands" \
"*::: :->jpg" \
&& ret=0
case $state in
    (jpg)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-jpg-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(bmp)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__bmp_commands" \
"*::: :->bmp" \
&& ret=0
case $state in
    (bmp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-bmp-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ico)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__ico_commands" \
"*::: :->ico" \
&& ret=0
case $state in
    (ico)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-ico-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tiff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__tiff_commands" \
"*::: :->tiff" \
&& ret=0
case $state in
    (tiff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-tiff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tga)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__tga_commands" \
"*::: :->tga" \
&& ret=0
case $state in
    (tga)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-tga-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(pnm)
_arguments "${_arguments_options[@]}" \
'-f+[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'--format=[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'-e+[Set the encoding for storing the samples]: :(binary ascii)' \
'--encoding=[Set the encoding for storing the samples]: :(binary ascii)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__pnm_commands" \
"*::: :->pnm" \
&& ret=0
case $state in
    (pnm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-pnm-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__split__ff_commands" \
"*::: :->ff" \
&& ret=0
case $state in
    (ff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-split-ff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(combine)
_arguments "${_arguments_options[@]}" \
'-f+[Set the FPS]' \
'--fps=[Set the FPS]' \
'-q+[Set the frame quality (1-100)]' \
'--quality=[Set the frame quality (1-100)]' \
'-r+[Set the number of repetitions]' \
'--repeat=[Set the number of repetitions]' \
'-s+[Set the animation speed]' \
'--speed=[Set the animation speed]' \
'--cut-beginning=[Cut the beginning of the animation]' \
'--cut-end=[Cut the end of the animation]' \
'-d+[Set the directory to read frames]' \
'--dir=[Set the directory to read frames]' \
'--format=[Set the animation format]: :(gif apng)' \
'--gifski[Use the gifski encoder]' \
'--fast[Encode 3 times faster (gifski)]' \
'-n[Use frames in the order given]' \
'--no-sort[Use frames in the order given]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':frames -- Set the animation frames:_files' \
":: :_menyoki__make_commands" \
"*::: :->make" \
&& ret=0
case $state in
    (make)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-make-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(make)
_arguments "${_arguments_options[@]}" \
'-f+[Set the FPS]' \
'--fps=[Set the FPS]' \
'-q+[Set the frame quality (1-100)]' \
'--quality=[Set the frame quality (1-100)]' \
'-r+[Set the number of repetitions]' \
'--repeat=[Set the number of repetitions]' \
'-s+[Set the animation speed]' \
'--speed=[Set the animation speed]' \
'--cut-beginning=[Cut the beginning of the animation]' \
'--cut-end=[Cut the end of the animation]' \
'-d+[Set the directory to read frames]' \
'--dir=[Set the directory to read frames]' \
'--format=[Set the animation format]: :(gif apng)' \
'--gifski[Use the gifski encoder]' \
'--fast[Encode 3 times faster (gifski)]' \
'-n[Use frames in the order given]' \
'--no-sort[Use frames in the order given]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':frames -- Set the animation frames:_files' \
":: :_menyoki__make_commands" \
"*::: :->make" \
&& ret=0
case $state in
    (make)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-make-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(screenshot)
_arguments "${_arguments_options[@]}" \
'--action-keys=[Set the action keys]' \
'--cancel-keys=[Set the cancel keys]' \
'-b+[Set the border width]' \
'--border=[Set the border width]' \
'-p+[Set the capture area padding]' \
'--padding=[Set the capture area padding]' \
'-s+[Set the capture area size]' \
'--size=[Set the capture area size]' \
'-d+[Set the duration for recording]' \
'--duration=[Set the duration for recording]' \
'-c+[Set the countdown before capturing]' \
'--countdown=[Set the countdown before capturing]' \
'-t+[Set the timeout for window selection]' \
'--timeout=[Set the timeout for window selection]' \
'-i+[Set the refresh interval for window selection]' \
'--interval=[Set the refresh interval for window selection]' \
'--font=[Set the font to use for window selection]' \
'--monitor=[Set the monitor to capture as root window]' \
'-r[Capture the root window]' \
'--root[Capture the root window]' \
'(-r --root)-f[Capture the focused window]' \
'(-r --root)--focus[Capture the focused window]' \
'--select[Select the window to capture]' \
'--parent[Capture the parent of the window]' \
'--with-alpha[Capture with the alpha channel]' \
'--no-keys[Disable the action keys while recording]' \
'-m[Select the window with mouse click]' \
'--mouse[Select the window with mouse click]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::command -- Set the command to run:_files' \
":: :_menyoki__capture_commands" \
"*::: :->capture" \
&& ret=0
case $state in
    (capture)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-command-$line[2]:"
        case $line[2] in
            (png)
_arguments "${_arguments_options[@]}" \
'-c+[Set the compression level]: :(default fast best huffman rle)' \
'--compression=[Set the compression level]: :(default fast best huffman rle)' \
'-f+[Set the filter algorithm]: :(none sub up avg paeth)' \
'--filter=[Set the filter algorithm]: :(none sub up avg paeth)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__png_commands" \
"*::: :->png" \
&& ret=0
case $state in
    (png)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-png-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(jpg)
_arguments "${_arguments_options[@]}" \
'-q+[Set the image quality (1-100)]' \
'--quality=[Set the image quality (1-100)]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__jpg_commands" \
"*::: :->jpg" \
&& ret=0
case $state in
    (jpg)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-jpg-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(bmp)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__bmp_commands" \
"*::: :->bmp" \
&& ret=0
case $state in
    (bmp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-bmp-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ico)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__ico_commands" \
"*::: :->ico" \
&& ret=0
case $state in
    (ico)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-ico-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tiff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__tiff_commands" \
"*::: :->tiff" \
&& ret=0
case $state in
    (tiff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-tiff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tga)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__tga_commands" \
"*::: :->tga" \
&& ret=0
case $state in
    (tga)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-tga-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(pnm)
_arguments "${_arguments_options[@]}" \
'-f+[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'--format=[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'-e+[Set the encoding for storing the samples]: :(binary ascii)' \
'--encoding=[Set the encoding for storing the samples]: :(binary ascii)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__pnm_commands" \
"*::: :->pnm" \
&& ret=0
case $state in
    (pnm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-pnm-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__ff_commands" \
"*::: :->ff" \
&& ret=0
case $state in
    (ff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-ff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ss)
_arguments "${_arguments_options[@]}" \
'--action-keys=[Set the action keys]' \
'--cancel-keys=[Set the cancel keys]' \
'-b+[Set the border width]' \
'--border=[Set the border width]' \
'-p+[Set the capture area padding]' \
'--padding=[Set the capture area padding]' \
'-s+[Set the capture area size]' \
'--size=[Set the capture area size]' \
'-d+[Set the duration for recording]' \
'--duration=[Set the duration for recording]' \
'-c+[Set the countdown before capturing]' \
'--countdown=[Set the countdown before capturing]' \
'-t+[Set the timeout for window selection]' \
'--timeout=[Set the timeout for window selection]' \
'-i+[Set the refresh interval for window selection]' \
'--interval=[Set the refresh interval for window selection]' \
'--font=[Set the font to use for window selection]' \
'--monitor=[Set the monitor to capture as root window]' \
'-r[Capture the root window]' \
'--root[Capture the root window]' \
'(-r --root)-f[Capture the focused window]' \
'(-r --root)--focus[Capture the focused window]' \
'--select[Select the window to capture]' \
'--parent[Capture the parent of the window]' \
'--with-alpha[Capture with the alpha channel]' \
'--no-keys[Disable the action keys while recording]' \
'-m[Select the window with mouse click]' \
'--mouse[Select the window with mouse click]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::command -- Set the command to run:_files' \
":: :_menyoki__capture_commands" \
"*::: :->capture" \
&& ret=0
case $state in
    (capture)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-command-$line[2]:"
        case $line[2] in
            (png)
_arguments "${_arguments_options[@]}" \
'-c+[Set the compression level]: :(default fast best huffman rle)' \
'--compression=[Set the compression level]: :(default fast best huffman rle)' \
'-f+[Set the filter algorithm]: :(none sub up avg paeth)' \
'--filter=[Set the filter algorithm]: :(none sub up avg paeth)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__png_commands" \
"*::: :->png" \
&& ret=0
case $state in
    (png)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-png-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(jpg)
_arguments "${_arguments_options[@]}" \
'-q+[Set the image quality (1-100)]' \
'--quality=[Set the image quality (1-100)]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__jpg_commands" \
"*::: :->jpg" \
&& ret=0
case $state in
    (jpg)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-jpg-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(bmp)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__bmp_commands" \
"*::: :->bmp" \
&& ret=0
case $state in
    (bmp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-bmp-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ico)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__ico_commands" \
"*::: :->ico" \
&& ret=0
case $state in
    (ico)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-ico-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tiff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__tiff_commands" \
"*::: :->tiff" \
&& ret=0
case $state in
    (tiff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-tiff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tga)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__tga_commands" \
"*::: :->tga" \
&& ret=0
case $state in
    (tga)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-tga-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(pnm)
_arguments "${_arguments_options[@]}" \
'-f+[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'--format=[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'-e+[Set the encoding for storing the samples]: :(binary ascii)' \
'--encoding=[Set the encoding for storing the samples]: :(binary ascii)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__pnm_commands" \
"*::: :->pnm" \
&& ret=0
case $state in
    (pnm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-pnm-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__ff_commands" \
"*::: :->ff" \
&& ret=0
case $state in
    (ff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-ff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(capture)
_arguments "${_arguments_options[@]}" \
'--action-keys=[Set the action keys]' \
'--cancel-keys=[Set the cancel keys]' \
'-b+[Set the border width]' \
'--border=[Set the border width]' \
'-p+[Set the capture area padding]' \
'--padding=[Set the capture area padding]' \
'-s+[Set the capture area size]' \
'--size=[Set the capture area size]' \
'-d+[Set the duration for recording]' \
'--duration=[Set the duration for recording]' \
'-c+[Set the countdown before capturing]' \
'--countdown=[Set the countdown before capturing]' \
'-t+[Set the timeout for window selection]' \
'--timeout=[Set the timeout for window selection]' \
'-i+[Set the refresh interval for window selection]' \
'--interval=[Set the refresh interval for window selection]' \
'--font=[Set the font to use for window selection]' \
'--monitor=[Set the monitor to capture as root window]' \
'-r[Capture the root window]' \
'--root[Capture the root window]' \
'(-r --root)-f[Capture the focused window]' \
'(-r --root)--focus[Capture the focused window]' \
'--select[Select the window to capture]' \
'--parent[Capture the parent of the window]' \
'--with-alpha[Capture with the alpha channel]' \
'--no-keys[Disable the action keys while recording]' \
'-m[Select the window with mouse click]' \
'--mouse[Select the window with mouse click]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::command -- Set the command to run:_files' \
":: :_menyoki__capture_commands" \
"*::: :->capture" \
&& ret=0
case $state in
    (capture)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-command-$line[2]:"
        case $line[2] in
            (png)
_arguments "${_arguments_options[@]}" \
'-c+[Set the compression level]: :(default fast best huffman rle)' \
'--compression=[Set the compression level]: :(default fast best huffman rle)' \
'-f+[Set the filter algorithm]: :(none sub up avg paeth)' \
'--filter=[Set the filter algorithm]: :(none sub up avg paeth)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__png_commands" \
"*::: :->png" \
&& ret=0
case $state in
    (png)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-png-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(jpg)
_arguments "${_arguments_options[@]}" \
'-q+[Set the image quality (1-100)]' \
'--quality=[Set the image quality (1-100)]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__jpg_commands" \
"*::: :->jpg" \
&& ret=0
case $state in
    (jpg)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-jpg-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(bmp)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__bmp_commands" \
"*::: :->bmp" \
&& ret=0
case $state in
    (bmp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-bmp-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ico)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__ico_commands" \
"*::: :->ico" \
&& ret=0
case $state in
    (ico)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-ico-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tiff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__tiff_commands" \
"*::: :->tiff" \
&& ret=0
case $state in
    (tiff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-tiff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tga)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__tga_commands" \
"*::: :->tga" \
&& ret=0
case $state in
    (tga)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-tga-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(pnm)
_arguments "${_arguments_options[@]}" \
'-f+[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'--format=[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'-e+[Set the encoding for storing the samples]: :(binary ascii)' \
'--encoding=[Set the encoding for storing the samples]: :(binary ascii)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__pnm_commands" \
"*::: :->pnm" \
&& ret=0
case $state in
    (pnm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-pnm-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__capture__ff_commands" \
"*::: :->ff" \
&& ret=0
case $state in
    (ff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-capture-ff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(edit)
_arguments "${_arguments_options[@]}" \
'--crop=[Apply padding to crop the image]' \
'--resize=[Resize the image without keeping the aspect ratio]' \
'--ratio=[Resize the image proportionally by aspect ratio]' \
'--rotate=[Rotate the image (clockwise)]: :(90 180 270)' \
'--flip=[Flip the image]: :(horizontal vertical)' \
'--blur=[Blur the image]' \
'--hue=[Adjust the hue of the image]' \
'--contrast=[Adjust the contrast of the image]' \
'--brightness=[Adjust the brightness of the image]' \
'--filter=[Set the sampling filter for scaling]: :(nearest triangle catmull-rom gaussian lanczos3)' \
'--convert[Convert image using the given encoder]' \
'--grayscale[Convert image to grayscale]' \
'--invert[Invert the colors of the image]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':file -- Set the input file:_files' \
":: :_menyoki__edit_commands" \
"*::: :->edit" \
&& ret=0
case $state in
    (edit)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-command-$line[2]:"
        case $line[2] in
            (gif)
_arguments "${_arguments_options[@]}" \
'-f+[Set the FPS]' \
'--fps=[Set the FPS]' \
'-q+[Set the frame quality (1-100)]' \
'--quality=[Set the frame quality (1-100)]' \
'-r+[Set the number of repetitions]' \
'--repeat=[Set the number of repetitions]' \
'-s+[Set the animation speed]' \
'--speed=[Set the animation speed]' \
'--cut-beginning=[Cut the beginning of the animation]' \
'--cut-end=[Cut the end of the animation]' \
'-d+[Set the directory to read frames]' \
'--dir=[Set the directory to read frames]' \
'--format=[Set the animation format]: :(gif apng)' \
'--gifski[Use the gifski encoder]' \
'--fast[Encode 3 times faster (gifski)]' \
'-n[Use frames in the order given]' \
'--no-sort[Use frames in the order given]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::frames -- Set the animation frames:_files' \
":: :_menyoki__edit__gif_commands" \
"*::: :->gif" \
&& ret=0
case $state in
    (gif)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-gif-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(apng)
_arguments "${_arguments_options[@]}" \
'-f+[Set the FPS]' \
'--fps=[Set the FPS]' \
'-q+[Set the frame quality (1-100)]' \
'--quality=[Set the frame quality (1-100)]' \
'-r+[Set the number of repetitions]' \
'--repeat=[Set the number of repetitions]' \
'-s+[Set the animation speed]' \
'--speed=[Set the animation speed]' \
'--cut-beginning=[Cut the beginning of the animation]' \
'--cut-end=[Cut the end of the animation]' \
'-d+[Set the directory to read frames]' \
'--dir=[Set the directory to read frames]' \
'--format=[Set the animation format]: :(gif apng)' \
'--gifski[Use the gifski encoder]' \
'--fast[Encode 3 times faster (gifski)]' \
'-n[Use frames in the order given]' \
'--no-sort[Use frames in the order given]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::frames -- Set the animation frames:_files' \
":: :_menyoki__edit__apng_commands" \
"*::: :->apng" \
&& ret=0
case $state in
    (apng)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-apng-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(png)
_arguments "${_arguments_options[@]}" \
'-c+[Set the compression level]: :(default fast best huffman rle)' \
'--compression=[Set the compression level]: :(default fast best huffman rle)' \
'-f+[Set the filter algorithm]: :(none sub up avg paeth)' \
'--filter=[Set the filter algorithm]: :(none sub up avg paeth)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__png_commands" \
"*::: :->png" \
&& ret=0
case $state in
    (png)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-png-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(jpg)
_arguments "${_arguments_options[@]}" \
'-q+[Set the image quality (1-100)]' \
'--quality=[Set the image quality (1-100)]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__jpg_commands" \
"*::: :->jpg" \
&& ret=0
case $state in
    (jpg)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-jpg-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(bmp)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__bmp_commands" \
"*::: :->bmp" \
&& ret=0
case $state in
    (bmp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-bmp-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ico)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__ico_commands" \
"*::: :->ico" \
&& ret=0
case $state in
    (ico)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-ico-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tiff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__tiff_commands" \
"*::: :->tiff" \
&& ret=0
case $state in
    (tiff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-tiff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(tga)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__tga_commands" \
"*::: :->tga" \
&& ret=0
case $state in
    (tga)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-tga-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(pnm)
_arguments "${_arguments_options[@]}" \
'-f+[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'--format=[Set the PNM format]: :(bitmap graymap pixmap arbitrary)' \
'-e+[Set the encoding for storing the samples]: :(binary ascii)' \
'--encoding=[Set the encoding for storing the samples]: :(binary ascii)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__pnm_commands" \
"*::: :->pnm" \
&& ret=0
case $state in
    (pnm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-pnm-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(ff)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_menyoki__edit__ff_commands" \
"*::: :->ff" \
&& ret=0
case $state in
    (ff)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-edit-ff-command-$line[1]:"
        case $line[1] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(inspect)
_arguments "${_arguments_options[@]}" \
'-t+[Set the time zone of the report]: :(utc local)' \
'--time-zone=[Set the time zone of the report]: :(utc local)' \
'--timestamp[Use Unix timestamp for report dates]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':file -- Set the image file:_files' \
":: :_menyoki__analyze_commands" \
"*::: :->analyze" \
&& ret=0
case $state in
    (analyze)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-analyze-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(analyze)
_arguments "${_arguments_options[@]}" \
'-t+[Set the time zone of the report]: :(utc local)' \
'--time-zone=[Set the time zone of the report]: :(utc local)' \
'--timestamp[Use Unix timestamp for report dates]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':file -- Set the image file:_files' \
":: :_menyoki__analyze_commands" \
"*::: :->analyze" \
&& ret=0
case $state in
    (analyze)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:menyoki-analyze-command-$line[2]:"
        case $line[2] in
            (out)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" \
'-d+[Add formatted date/time to the file name]' \
'--date=[Add formatted date/time to the file name]' \
'-e[Always save the file with an extension]' \
'--with-extension[Always save the file with an extension]' \
'-t[Add Unix timestamp to the file name]' \
'--timestamp[Add Unix timestamp to the file name]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- Set the output file:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(view)
_arguments "${_arguments_options[@]}" \
'-t[Display transparent image with transparent background]' \
'--transparent[Display transparent image with transparent background]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
':file -- Set the input file:_files' \
&& ret=0
;;
(misc)
_arguments "${_arguments_options[@]}" \
'-g+[Generate completions for the specified shell]: :(bash fish zsh powershell elvish)' \
'--gen-completions=[Generate completions for the specified shell]: :(bash fish zsh powershell elvish)' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_menyoki_commands] )) ||
_menyoki_commands() {
    local commands; commands=(
        "record:Record an animation" \
"split:Split an animation into frames" \
"make:Make an animation from frames" \
"capture:Capture an image" \
"edit:Edit an image" \
"analyze:Analyze an image" \
"view:View an image" \
"misc:Perfom miscellaneous operations" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki commands' commands "$@"
}
(( $+functions[_menyoki__analyze_commands] )) ||
_menyoki__analyze_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki analyze commands' commands "$@"
}
(( $+functions[_menyoki__edit__apng_commands] )) ||
_menyoki__edit__apng_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit apng commands' commands "$@"
}
(( $+functions[_menyoki__record__apng_commands] )) ||
_menyoki__record__apng_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki record apng commands' commands "$@"
}
(( $+functions[_menyoki__capture__bmp_commands] )) ||
_menyoki__capture__bmp_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture bmp commands' commands "$@"
}
(( $+functions[_menyoki__edit__bmp_commands] )) ||
_menyoki__edit__bmp_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit bmp commands' commands "$@"
}
(( $+functions[_menyoki__split__bmp_commands] )) ||
_menyoki__split__bmp_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split bmp commands' commands "$@"
}
(( $+functions[_menyoki__capture_commands] )) ||
_menyoki__capture_commands() {
    local commands; commands=(
        "png:Use the PNG encoder" \
"jpg:Use the JPG encoder" \
"bmp:Use the BMP encoder" \
"ico:Use the ICO encoder" \
"tiff:Use the TIFF encoder" \
"tga:Use the TGA encoder" \
"pnm:Use the PNM encoder" \
"ff:Use the farbfeld encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture commands' commands "$@"
}
(( $+functions[_combine_commands] )) ||
_combine_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'combine commands' commands "$@"
}
(( $+functions[_menyoki__edit_commands] )) ||
_menyoki__edit_commands() {
    local commands; commands=(
        "gif:Use the GIF encoder" \
"apng:Use the APNG encoder" \
"png:Use the PNG encoder" \
"jpg:Use the JPG encoder" \
"bmp:Use the BMP encoder" \
"ico:Use the ICO encoder" \
"tiff:Use the TIFF encoder" \
"tga:Use the TGA encoder" \
"pnm:Use the PNM encoder" \
"ff:Use the farbfeld encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit commands' commands "$@"
}
(( $+functions[_extract_commands] )) ||
_extract_commands() {
    local commands; commands=(
        "png:Use the PNG encoder" \
"jpg:Use the JPG encoder" \
"bmp:Use the BMP encoder" \
"ico:Use the ICO encoder" \
"tiff:Use the TIFF encoder" \
"tga:Use the TGA encoder" \
"pnm:Use the PNM encoder" \
"ff:Use the farbfeld encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'extract commands' commands "$@"
}
(( $+functions[_menyoki__capture__ff_commands] )) ||
_menyoki__capture__ff_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture ff commands' commands "$@"
}
(( $+functions[_menyoki__edit__ff_commands] )) ||
_menyoki__edit__ff_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit ff commands' commands "$@"
}
(( $+functions[_menyoki__split__ff_commands] )) ||
_menyoki__split__ff_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split ff commands' commands "$@"
}
(( $+functions[_menyoki__edit__gif_commands] )) ||
_menyoki__edit__gif_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit gif commands' commands "$@"
}
(( $+functions[_menyoki__record__gif_commands] )) ||
_menyoki__record__gif_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki record gif commands' commands "$@"
}
(( $+functions[_menyoki__analyze__help_commands] )) ||
_menyoki__analyze__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki analyze help commands' commands "$@"
}
(( $+functions[_menyoki__capture__bmp__help_commands] )) ||
_menyoki__capture__bmp__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture bmp help commands' commands "$@"
}
(( $+functions[_menyoki__capture__ff__help_commands] )) ||
_menyoki__capture__ff__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture ff help commands' commands "$@"
}
(( $+functions[_menyoki__capture__help_commands] )) ||
_menyoki__capture__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture help commands' commands "$@"
}
(( $+functions[_menyoki__capture__ico__help_commands] )) ||
_menyoki__capture__ico__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture ico help commands' commands "$@"
}
(( $+functions[_menyoki__capture__jpg__help_commands] )) ||
_menyoki__capture__jpg__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture jpg help commands' commands "$@"
}
(( $+functions[_menyoki__capture__png__help_commands] )) ||
_menyoki__capture__png__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture png help commands' commands "$@"
}
(( $+functions[_menyoki__capture__pnm__help_commands] )) ||
_menyoki__capture__pnm__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture pnm help commands' commands "$@"
}
(( $+functions[_menyoki__capture__tga__help_commands] )) ||
_menyoki__capture__tga__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture tga help commands' commands "$@"
}
(( $+functions[_menyoki__capture__tiff__help_commands] )) ||
_menyoki__capture__tiff__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture tiff help commands' commands "$@"
}
(( $+functions[_menyoki__edit__apng__help_commands] )) ||
_menyoki__edit__apng__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit apng help commands' commands "$@"
}
(( $+functions[_menyoki__edit__bmp__help_commands] )) ||
_menyoki__edit__bmp__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit bmp help commands' commands "$@"
}
(( $+functions[_menyoki__edit__ff__help_commands] )) ||
_menyoki__edit__ff__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit ff help commands' commands "$@"
}
(( $+functions[_menyoki__edit__gif__help_commands] )) ||
_menyoki__edit__gif__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit gif help commands' commands "$@"
}
(( $+functions[_menyoki__edit__help_commands] )) ||
_menyoki__edit__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit help commands' commands "$@"
}
(( $+functions[_menyoki__edit__ico__help_commands] )) ||
_menyoki__edit__ico__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit ico help commands' commands "$@"
}
(( $+functions[_menyoki__edit__jpg__help_commands] )) ||
_menyoki__edit__jpg__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit jpg help commands' commands "$@"
}
(( $+functions[_menyoki__edit__png__help_commands] )) ||
_menyoki__edit__png__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit png help commands' commands "$@"
}
(( $+functions[_menyoki__edit__pnm__help_commands] )) ||
_menyoki__edit__pnm__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit pnm help commands' commands "$@"
}
(( $+functions[_menyoki__edit__tga__help_commands] )) ||
_menyoki__edit__tga__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit tga help commands' commands "$@"
}
(( $+functions[_menyoki__edit__tiff__help_commands] )) ||
_menyoki__edit__tiff__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit tiff help commands' commands "$@"
}
(( $+functions[_menyoki__help_commands] )) ||
_menyoki__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki help commands' commands "$@"
}
(( $+functions[_menyoki__make__help_commands] )) ||
_menyoki__make__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki make help commands' commands "$@"
}
(( $+functions[_menyoki__record__apng__help_commands] )) ||
_menyoki__record__apng__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record apng help commands' commands "$@"
}
(( $+functions[_menyoki__record__gif__help_commands] )) ||
_menyoki__record__gif__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record gif help commands' commands "$@"
}
(( $+functions[_menyoki__record__help_commands] )) ||
_menyoki__record__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record help commands' commands "$@"
}
(( $+functions[_menyoki__split__bmp__help_commands] )) ||
_menyoki__split__bmp__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split bmp help commands' commands "$@"
}
(( $+functions[_menyoki__split__ff__help_commands] )) ||
_menyoki__split__ff__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split ff help commands' commands "$@"
}
(( $+functions[_menyoki__split__help_commands] )) ||
_menyoki__split__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split help commands' commands "$@"
}
(( $+functions[_menyoki__split__ico__help_commands] )) ||
_menyoki__split__ico__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split ico help commands' commands "$@"
}
(( $+functions[_menyoki__split__jpg__help_commands] )) ||
_menyoki__split__jpg__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split jpg help commands' commands "$@"
}
(( $+functions[_menyoki__split__png__help_commands] )) ||
_menyoki__split__png__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split png help commands' commands "$@"
}
(( $+functions[_menyoki__split__pnm__help_commands] )) ||
_menyoki__split__pnm__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split pnm help commands' commands "$@"
}
(( $+functions[_menyoki__split__tga__help_commands] )) ||
_menyoki__split__tga__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split tga help commands' commands "$@"
}
(( $+functions[_menyoki__split__tiff__help_commands] )) ||
_menyoki__split__tiff__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split tiff help commands' commands "$@"
}
(( $+functions[_menyoki__capture__ico_commands] )) ||
_menyoki__capture__ico_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture ico commands' commands "$@"
}
(( $+functions[_menyoki__edit__ico_commands] )) ||
_menyoki__edit__ico_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit ico commands' commands "$@"
}
(( $+functions[_menyoki__split__ico_commands] )) ||
_menyoki__split__ico_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split ico commands' commands "$@"
}
(( $+functions[_inspect_commands] )) ||
_inspect_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'inspect commands' commands "$@"
}
(( $+functions[_menyoki__capture__jpg_commands] )) ||
_menyoki__capture__jpg_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture jpg commands' commands "$@"
}
(( $+functions[_menyoki__edit__jpg_commands] )) ||
_menyoki__edit__jpg_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit jpg commands' commands "$@"
}
(( $+functions[_menyoki__split__jpg_commands] )) ||
_menyoki__split__jpg_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split jpg commands' commands "$@"
}
(( $+functions[_menyoki__make_commands] )) ||
_menyoki__make_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki make commands' commands "$@"
}
(( $+functions[_menyoki__misc_commands] )) ||
_menyoki__misc_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki misc commands' commands "$@"
}
(( $+functions[_menyoki__analyze__out_commands] )) ||
_menyoki__analyze__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki analyze out commands' commands "$@"
}
(( $+functions[_menyoki__capture__bmp__out_commands] )) ||
_menyoki__capture__bmp__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture bmp out commands' commands "$@"
}
(( $+functions[_menyoki__capture__ff__out_commands] )) ||
_menyoki__capture__ff__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture ff out commands' commands "$@"
}
(( $+functions[_menyoki__capture__ico__out_commands] )) ||
_menyoki__capture__ico__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture ico out commands' commands "$@"
}
(( $+functions[_menyoki__capture__jpg__out_commands] )) ||
_menyoki__capture__jpg__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture jpg out commands' commands "$@"
}
(( $+functions[_menyoki__capture__out_commands] )) ||
_menyoki__capture__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture out commands' commands "$@"
}
(( $+functions[_menyoki__capture__png__out_commands] )) ||
_menyoki__capture__png__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture png out commands' commands "$@"
}
(( $+functions[_menyoki__capture__pnm__out_commands] )) ||
_menyoki__capture__pnm__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture pnm out commands' commands "$@"
}
(( $+functions[_menyoki__capture__tga__out_commands] )) ||
_menyoki__capture__tga__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture tga out commands' commands "$@"
}
(( $+functions[_menyoki__capture__tiff__out_commands] )) ||
_menyoki__capture__tiff__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture tiff out commands' commands "$@"
}
(( $+functions[_menyoki__edit__apng__out_commands] )) ||
_menyoki__edit__apng__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit apng out commands' commands "$@"
}
(( $+functions[_menyoki__edit__bmp__out_commands] )) ||
_menyoki__edit__bmp__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit bmp out commands' commands "$@"
}
(( $+functions[_menyoki__edit__ff__out_commands] )) ||
_menyoki__edit__ff__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit ff out commands' commands "$@"
}
(( $+functions[_menyoki__edit__gif__out_commands] )) ||
_menyoki__edit__gif__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit gif out commands' commands "$@"
}
(( $+functions[_menyoki__edit__ico__out_commands] )) ||
_menyoki__edit__ico__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit ico out commands' commands "$@"
}
(( $+functions[_menyoki__edit__jpg__out_commands] )) ||
_menyoki__edit__jpg__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit jpg out commands' commands "$@"
}
(( $+functions[_menyoki__edit__out_commands] )) ||
_menyoki__edit__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit out commands' commands "$@"
}
(( $+functions[_menyoki__edit__png__out_commands] )) ||
_menyoki__edit__png__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit png out commands' commands "$@"
}
(( $+functions[_menyoki__edit__pnm__out_commands] )) ||
_menyoki__edit__pnm__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit pnm out commands' commands "$@"
}
(( $+functions[_menyoki__edit__tga__out_commands] )) ||
_menyoki__edit__tga__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit tga out commands' commands "$@"
}
(( $+functions[_menyoki__edit__tiff__out_commands] )) ||
_menyoki__edit__tiff__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit tiff out commands' commands "$@"
}
(( $+functions[_menyoki__make__out_commands] )) ||
_menyoki__make__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki make out commands' commands "$@"
}
(( $+functions[_menyoki__out_commands] )) ||
_menyoki__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki out commands' commands "$@"
}
(( $+functions[_menyoki__record__apng__out_commands] )) ||
_menyoki__record__apng__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record apng out commands' commands "$@"
}
(( $+functions[_menyoki__record__gif__out_commands] )) ||
_menyoki__record__gif__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record gif out commands' commands "$@"
}
(( $+functions[_menyoki__record__out_commands] )) ||
_menyoki__record__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record out commands' commands "$@"
}
(( $+functions[_menyoki__split__bmp__out_commands] )) ||
_menyoki__split__bmp__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split bmp out commands' commands "$@"
}
(( $+functions[_menyoki__split__ff__out_commands] )) ||
_menyoki__split__ff__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split ff out commands' commands "$@"
}
(( $+functions[_menyoki__split__ico__out_commands] )) ||
_menyoki__split__ico__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split ico out commands' commands "$@"
}
(( $+functions[_menyoki__split__jpg__out_commands] )) ||
_menyoki__split__jpg__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split jpg out commands' commands "$@"
}
(( $+functions[_menyoki__split__out_commands] )) ||
_menyoki__split__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split out commands' commands "$@"
}
(( $+functions[_menyoki__split__png__out_commands] )) ||
_menyoki__split__png__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split png out commands' commands "$@"
}
(( $+functions[_menyoki__split__pnm__out_commands] )) ||
_menyoki__split__pnm__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split pnm out commands' commands "$@"
}
(( $+functions[_menyoki__split__tga__out_commands] )) ||
_menyoki__split__tga__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split tga out commands' commands "$@"
}
(( $+functions[_menyoki__split__tiff__out_commands] )) ||
_menyoki__split__tiff__out_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split tiff out commands' commands "$@"
}
(( $+functions[_menyoki__capture__png_commands] )) ||
_menyoki__capture__png_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture png commands' commands "$@"
}
(( $+functions[_menyoki__edit__png_commands] )) ||
_menyoki__edit__png_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit png commands' commands "$@"
}
(( $+functions[_menyoki__split__png_commands] )) ||
_menyoki__split__png_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split png commands' commands "$@"
}
(( $+functions[_menyoki__capture__pnm_commands] )) ||
_menyoki__capture__pnm_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture pnm commands' commands "$@"
}
(( $+functions[_menyoki__edit__pnm_commands] )) ||
_menyoki__edit__pnm_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit pnm commands' commands "$@"
}
(( $+functions[_menyoki__split__pnm_commands] )) ||
_menyoki__split__pnm_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split pnm commands' commands "$@"
}
(( $+functions[_menyoki__record_commands] )) ||
_menyoki__record_commands() {
    local commands; commands=(
        "gif:Use the GIF encoder" \
"apng:Use the APNG encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki record commands' commands "$@"
}
(( $+functions[_menyoki__analyze__save_commands] )) ||
_menyoki__analyze__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki analyze save commands' commands "$@"
}
(( $+functions[_menyoki__capture__bmp__save_commands] )) ||
_menyoki__capture__bmp__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture bmp save commands' commands "$@"
}
(( $+functions[_menyoki__capture__ff__save_commands] )) ||
_menyoki__capture__ff__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture ff save commands' commands "$@"
}
(( $+functions[_menyoki__capture__ico__save_commands] )) ||
_menyoki__capture__ico__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture ico save commands' commands "$@"
}
(( $+functions[_menyoki__capture__jpg__save_commands] )) ||
_menyoki__capture__jpg__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture jpg save commands' commands "$@"
}
(( $+functions[_menyoki__capture__png__save_commands] )) ||
_menyoki__capture__png__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture png save commands' commands "$@"
}
(( $+functions[_menyoki__capture__pnm__save_commands] )) ||
_menyoki__capture__pnm__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture pnm save commands' commands "$@"
}
(( $+functions[_menyoki__capture__save_commands] )) ||
_menyoki__capture__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture save commands' commands "$@"
}
(( $+functions[_menyoki__capture__tga__save_commands] )) ||
_menyoki__capture__tga__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture tga save commands' commands "$@"
}
(( $+functions[_menyoki__capture__tiff__save_commands] )) ||
_menyoki__capture__tiff__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki capture tiff save commands' commands "$@"
}
(( $+functions[_menyoki__edit__apng__save_commands] )) ||
_menyoki__edit__apng__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit apng save commands' commands "$@"
}
(( $+functions[_menyoki__edit__bmp__save_commands] )) ||
_menyoki__edit__bmp__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit bmp save commands' commands "$@"
}
(( $+functions[_menyoki__edit__ff__save_commands] )) ||
_menyoki__edit__ff__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit ff save commands' commands "$@"
}
(( $+functions[_menyoki__edit__gif__save_commands] )) ||
_menyoki__edit__gif__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit gif save commands' commands "$@"
}
(( $+functions[_menyoki__edit__ico__save_commands] )) ||
_menyoki__edit__ico__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit ico save commands' commands "$@"
}
(( $+functions[_menyoki__edit__jpg__save_commands] )) ||
_menyoki__edit__jpg__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit jpg save commands' commands "$@"
}
(( $+functions[_menyoki__edit__png__save_commands] )) ||
_menyoki__edit__png__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit png save commands' commands "$@"
}
(( $+functions[_menyoki__edit__pnm__save_commands] )) ||
_menyoki__edit__pnm__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit pnm save commands' commands "$@"
}
(( $+functions[_menyoki__edit__save_commands] )) ||
_menyoki__edit__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit save commands' commands "$@"
}
(( $+functions[_menyoki__edit__tga__save_commands] )) ||
_menyoki__edit__tga__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit tga save commands' commands "$@"
}
(( $+functions[_menyoki__edit__tiff__save_commands] )) ||
_menyoki__edit__tiff__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki edit tiff save commands' commands "$@"
}
(( $+functions[_menyoki__make__save_commands] )) ||
_menyoki__make__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki make save commands' commands "$@"
}
(( $+functions[_menyoki__record__apng__save_commands] )) ||
_menyoki__record__apng__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record apng save commands' commands "$@"
}
(( $+functions[_menyoki__record__gif__save_commands] )) ||
_menyoki__record__gif__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record gif save commands' commands "$@"
}
(( $+functions[_menyoki__record__save_commands] )) ||
_menyoki__record__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki record save commands' commands "$@"
}
(( $+functions[_menyoki__split__bmp__save_commands] )) ||
_menyoki__split__bmp__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split bmp save commands' commands "$@"
}
(( $+functions[_menyoki__split__ff__save_commands] )) ||
_menyoki__split__ff__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split ff save commands' commands "$@"
}
(( $+functions[_menyoki__split__ico__save_commands] )) ||
_menyoki__split__ico__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split ico save commands' commands "$@"
}
(( $+functions[_menyoki__split__jpg__save_commands] )) ||
_menyoki__split__jpg__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split jpg save commands' commands "$@"
}
(( $+functions[_menyoki__split__png__save_commands] )) ||
_menyoki__split__png__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split png save commands' commands "$@"
}
(( $+functions[_menyoki__split__pnm__save_commands] )) ||
_menyoki__split__pnm__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split pnm save commands' commands "$@"
}
(( $+functions[_menyoki__split__save_commands] )) ||
_menyoki__split__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split save commands' commands "$@"
}
(( $+functions[_menyoki__split__tga__save_commands] )) ||
_menyoki__split__tga__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split tga save commands' commands "$@"
}
(( $+functions[_menyoki__split__tiff__save_commands] )) ||
_menyoki__split__tiff__save_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki split tiff save commands' commands "$@"
}
(( $+functions[_screenshot_commands] )) ||
_screenshot_commands() {
    local commands; commands=(
        "png:Use the PNG encoder" \
"jpg:Use the JPG encoder" \
"bmp:Use the BMP encoder" \
"ico:Use the ICO encoder" \
"tiff:Use the TIFF encoder" \
"tga:Use the TGA encoder" \
"pnm:Use the PNM encoder" \
"ff:Use the farbfeld encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'screenshot commands' commands "$@"
}
(( $+functions[_menyoki__split_commands] )) ||
_menyoki__split_commands() {
    local commands; commands=(
        "png:Use the PNG encoder" \
"jpg:Use the JPG encoder" \
"bmp:Use the BMP encoder" \
"ico:Use the ICO encoder" \
"tiff:Use the TIFF encoder" \
"tga:Use the TGA encoder" \
"pnm:Use the PNM encoder" \
"ff:Use the farbfeld encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split commands' commands "$@"
}
(( $+functions[_ss_commands] )) ||
_ss_commands() {
    local commands; commands=(
        "png:Use the PNG encoder" \
"jpg:Use the JPG encoder" \
"bmp:Use the BMP encoder" \
"ico:Use the ICO encoder" \
"tiff:Use the TIFF encoder" \
"tga:Use the TGA encoder" \
"pnm:Use the PNM encoder" \
"ff:Use the farbfeld encoder" \
"save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'ss commands' commands "$@"
}
(( $+functions[_menyoki__capture__tga_commands] )) ||
_menyoki__capture__tga_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture tga commands' commands "$@"
}
(( $+functions[_menyoki__edit__tga_commands] )) ||
_menyoki__edit__tga_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit tga commands' commands "$@"
}
(( $+functions[_menyoki__split__tga_commands] )) ||
_menyoki__split__tga_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split tga commands' commands "$@"
}
(( $+functions[_menyoki__capture__tiff_commands] )) ||
_menyoki__capture__tiff_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki capture tiff commands' commands "$@"
}
(( $+functions[_menyoki__edit__tiff_commands] )) ||
_menyoki__edit__tiff_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki edit tiff commands' commands "$@"
}
(( $+functions[_menyoki__split__tiff_commands] )) ||
_menyoki__split__tiff_commands() {
    local commands; commands=(
        "save:Save the output file(s)" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'menyoki split tiff commands' commands "$@"
}
(( $+functions[_menyoki__view_commands] )) ||
_menyoki__view_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'menyoki view commands' commands "$@"
}

_menyoki "$@"