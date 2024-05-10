<h1>Woordle - Rust Wordle Assistant</h1>
<div>
  <p>This is a program to recommend you words, based on given letters.</p>
</div>

<h2>How to use</h2>
<div>
  <p>The program will first ask you to input your word.</p>
  <p>Since you don't know what the word is, you replace the unknown words with a number (e.g. 0, the numbers themselfs are just placeholders with no effect).</p>
  <p>Letters you know are at the right spot (marked green in Wordle) are entered in uppercase. Letters, you know are in the word but don't know the right position of (green in Wordle) are entered in lowercase.</p>

  <p><i>Here is an example, let's say the hidden word is 'SETUP':</i></p>
  <ul>
    <li>Inputing first word in Wordle: <b>P E A C E</b></li>
    <li><b>A</b>, <b>C</b> and <b>E</b> are wrong, <b>E</b> is correct and at the right position and <b>P</b> is at the wrong position</li>
    <li>Entering letters in program: <b>pE000</b></li>
    <li>Program gives list of posible words, choosing <b>'depth'</b></li>
    <li><b>D</b> and <b>H</b> are wrong, <b>E</b> is correct, <b>P</b> and <b>T</b> are at the wrong position</li>
    <li>Again, enter <b>0Ept0</b>, choosing word '<b>tempo</b>'</li>
    <li><b>M</b> and <b>O</b> are wrong, <b>E</b> is still the only correct letter and <b>P</b> and <b>T</b> are at the wrong position</li>
    <li>Entering <b>tE0p0</b>, choosing word '<b>setup</b>'</li>
    <li>CORRECT! Lucky guess ;)</li>
  </ul>
</div>

<h2>About</h2>
<div>
  <p>This program contains all 14855 5-letter words found in Wordle (as of May 2024). The program is written entirely in Rust with no dependencies, so just the standard library! Still, this program is not perfect and was just a fun excercise. I hope you can have some fun with it!</p>
</div>

<hr>

<p><i>Note: Program is still in development; there are still some bugs, but the program works in almost every case.</i></p>
