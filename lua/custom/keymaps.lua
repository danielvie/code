-- facilities
vim.keymap.set('n', '<leader>w', '<cmd>w<cr>')
vim.keymap.set('n', '<leader>q', '<cmd>q<cr>')
vim.keymap.set('n', '<leader>x', '<cmd>bd<cr>')
vim.keymap.set('n', '<leader>pv', '<cmd>Ex<cr>')
vim.keymap.set('n', '<leader>0', '<cmd>cd %:p:h<cr>')
vim.keymap.set('n', '<leader>v', '<cmd>!make<cr>')
vim.keymap.set('n', '<s-l>', '<cmd>bn<cr>')
vim.keymap.set('n', '<s-h>', '<cmd>bp<cr>')

vim.keymap.set('n', '<leader>no', '<cmd>e $userprofile/desktop/notes.md<cr>')
vim.keymap.set('n', '<s-r>', 'za')
vim.keymap.set('n', '<leader>t', 'ggVG')

-- quickfix list
vim.keymap.set('n', '<a-y>', '<cmd>cfirst<cr>', { desc = 'first quickfix item' })
vim.keymap.set('n', '<a-i>', '<cmd>cnext<cr>', { desc = 'next quickfix item' })
vim.keymap.set('n', '<a-u>', '<cmd>cprev<cr>', { desc = 'prev quickfix item' })
vim.keymap.set('n', '<a-o>', '<cmd>clast<cr>', { desc = 'last quickfix item' })

vim.keymap.set('n', '<leader>ks', '<cmd>e $localappdata/nvim/lua/custom/keymaps.lua<cr>', { desc = 'goto `Keymaps.lua`' })

-- macros
vim.keymap.set('n', '<s-q>', '@q', { desc = 'run macro @q' })
vim.keymap.set('n', '<s-w>', '@w', { desc = 'run macro @w' })
vim.keymap.set('n', '<s-e>', '@w', { desc = 'run macro @e' })

-- fix windows
vim.keymap.set('n', '<c-q>', '<c-v>', { desc = 'fix windows for <c-v>' })

-- operators
vim.keymap.set('o', '<s-l>', '$', { desc = 'operators to end of line' })
vim.keymap.set('o', '<s-h>', '^', { desc = 'operators to begining of line' })
