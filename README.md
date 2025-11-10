### 📚 woro
A game which allows you to memorise foreign words and their translations easier

##  Features
- Add words manually or import from TXT  
- Level system (1 → 5)
  - Correct → level up
  - Wrong → level down
- Random next word; progress bar; end screen when all reach level 5
- JSON auto‑save (words and levels survive restarts)
- Clean GUI (egui) with keyboard‑friendly input flow


## Installation
# On Windows
Download "woro.exe" and launch it.
# On Linux
Download the appimage and launch it.

##  System Requirements
- Windows: 10/11 (x64)(8.1 likely works).  
  Windows 7 is not supported (missing api‑ms‑win APIs). 8.1 likely works.
- Linux: x86_64 (glibc). Tested on Gentoo; should work on the other distros.

##  How to Use
1. Start the app.
2. Add words manually (foreign + translation), or:
3. Import a TXT list:
   - Each line: `foreign translation`
     Note: If you want woro to ask the translation's foreign word, just give the words and translations in reverse order.
4. Go to Game:
   - Type the translation and press Enter or click “Check”
   - Correct → level up; Wrong → shows the correct translation and moves on
   - Progress bar shows mastered/total
   - When all reach level 5 (it generally takes a bit of time), you’ll see an end screen which celebrates you 🥳.

## Contributing
While I appreciate interest in this project, please note that as a student with limited time, I may not be able to review pull requests regularly. So don't expect fast feedback 😓. This is primarily a personal project for my portfolio.

## License
MIT 


