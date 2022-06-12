let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd /mnt/Projects/Rust/math-ast
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +150 src/main.rs
badd +1241 term:///mnt/Projects/Rust/math-ast//57222:/usr/bin/zsh
badd +62 ~/.config/nvim/lua/config.lua
argglobal
%argdel
$argadd src/main.rs
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit src/main.rs
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=1
setlocal fml=1
setlocal fdn=20
setlocal fen
14
normal! zo
16
normal! zo
18
normal! zo
56
normal! zo
57
normal! zo
58
normal! zo
66
normal! zo
89
normal! zo
97
normal! zo
103
normal! zo
104
normal! zo
111
normal! zo
125
normal! zo
141
normal! zo
170
normal! zo
171
normal! zo
176
normal! zo
177
normal! zo
178
normal! zo
183
normal! zo
190
normal! zo
198
normal! zo
200
normal! zo
204
normal! zo
209
normal! zo
210
normal! zo
208
normal! zo
209
normal! zo
227
normal! zo
231
normal! zo
233
normal! zo
176
normal! zc
242
normal! zo
250
normal! zo
let s:l = 150 - ((27 * winheight(0) + 28) / 56)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 150
normal! 061|
tabnext
argglobal
if bufexists(fnamemodify("term:///mnt/Projects/Rust/math-ast//57222:/usr/bin/zsh", ":p")) | buffer term:///mnt/Projects/Rust/math-ast//57222:/usr/bin/zsh | else | edit term:///mnt/Projects/Rust/math-ast//57222:/usr/bin/zsh | endif
if &buftype ==# 'terminal'
  silent file term:///mnt/Projects/Rust/math-ast//57222:/usr/bin/zsh
endif
balt src/main.rs
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1697 - ((55 * winheight(0) + 28) / 56)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1697
normal! 02|
tabnext 1
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
