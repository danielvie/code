" Have j and k navigate visual lines rather than logical ones
nmap j gj
nmap k gk

" I like using H and L for beginning/end of line
nmap H ^
nmap L $

nmap Q @q
nmap W @w

" operators
omap L $
omap H ^

" Quickly remove search highlights
nmap <esc> :nohl<CR>

" Yank to system clipboard
set clipboard=unnamed

" Go back and forward with Ctrl+O and Ctrl+I
" (make sure to remove default Obsidian shortcuts for these to work)
exmap back obcommand app:go-back
nmap <C-o> :back<CR>
exmap forward obcommand app:go-forward
nmap <C-i> :forward<CR>

" Save with <space>w
unmap <Space>
noremap <Space>w :w<CR>

" exmap unfoldall obcommand editor:unfold-all
nmap zr :unfoldall<CR>

" exmap foldall obcommand editor:fold-all
nmap zm :foldall<CR>

nmap <A-t> ggVG
