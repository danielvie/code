## GRUB TIPS

set pager to grub

```bash
set pager=1
```

list folders in partition

```bash
ls (hd0,gpt1)/...
```

set root and loads the normal module
```bash
set prefix=(hd0,gpt2)/boot/grub
set root=(hd0,gpt2)
insmod normal
normal
```
