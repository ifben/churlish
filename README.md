## churlish: a wordle guesser

churlish is a simple commandline wordle guesser that tracks multiple wordle guesses to provide an accurate list of valid guesses. It makes use of [fancy-regex](https://crates.io/crates/fancy-regex) and uses positive lookaheads for precise word matching. Please note: I wrote this tool mostly to practice completing something (relatively) useful in Rust, which I am very much in the process of learning. There is likely to be a lot of bad, non-idiomatic code here, which I will work on improving. But, in all my tests, it is working! 

### Installation

Since this is just a tiny CLI program made for myself, there are no real plans to package it beyond providing the source here. So, feel free to build it from source.

### Usage

churlish works by taking the results of your wordle guess described in a simple pattern and returning a list of valid guesses. For wrong letters (gray), these are inputted as a ```?```. For misplaced letters (yellow), these are inputted in their position as lowercase letters. For correctly placed letters, these are inputted in their position as uppercase letters. For example, consider a wordle first guess result of: 

![example wordle result](https://i.imgur.com/QcKXcpM.png)

This would be entered into ```churlish``` as ```-1 '?r?T?'```, with ```CAE``` given to the ```--wrong``` or ```-w``` argument.  

**Note**: question marks in most shells will match filenames, so inputting ```?????``` into ```churlish``` may input the name of a 5-letter file into ```churlish```. This can be fixed by simply single-quoting your pattern as, for example, ```'?????'```. This will of course depend on where ```churlish``` is located locally. 

```churlish``` was built for wordle, so it supports entering 4 possible guess patterns. It can be used in wordle derivatives like quordle, but you must take care to not enter impossible pattern options. 

### Example

Let's try an interactive example. Let's say we follow [3Blue1Brown's advice](https://www.youtube.com/watch?v=fRed0Xmc2Wg) and choose **CRATE** as our first guess. We then get the following result: 

![first demo guess](https://i.imgur.com/zpwwHB4.png)

Our pattern into ```churlish``` would then be: ```???t?``` with ```CRAE``` to give to the ```--wrong``` argument: 

```churlish -1 '???t?' -w crae```

Not a great first guess this time, and ```churlish``` gives us 369 valid guesses:

<details>
    <summary>churlish output</summary>

```
1: shtup
2: thins
3: tunny
4: night
5: studs
6: squit
7: toxin
8: tossy
9: phpht
10: tifos
11: guyot
12: stims
13: bitsy
14: smolt
15: tofus
16: smoot
17: glout
18: stool
19: gothy
20: situp
21: tight
22: jotun
23: flint
24: toyon
25: tikis
26: withy
27: mount
28: split
29: stoln
30: tohos
31: stook
32: glost
33: spout
34: dight
35: stoit
36: tolly
37: poult
38: slipt
39: pight
40: obiit
41: mitis
42: tuffs
43: ingot
44: tommy
45: thoft
46: zitis
47: poupt
48: guilt
49: thill
50: litho
51: tutus
52: moust
53: limit
54: twill
55: tholi
56: qubit
57: kydst
58: tomos
59: kutis
60: intil
61: vomit
62: innit
63: point
64: stunk
65: timon
66: tummy
67: tumps
68: mutis
69: fight
70: ytost
71: gigot
72: tubby
73: stown
74: sting
75: tunds
76: stowp
77: sloot
78: tizzy
79: tongs
80: smout
81: putid
82: giust
83: thymy
84: tusks
85: doilt
86: moult
87: topis
88: stull
89: toils
90: foist
91: quilt
92: shtik
93: still
94: buist
95: potsy
96: moist
97: stiff
98: tumpy
99: tombs
100: donut
101: intis
102: toshy
103: knout
104: toyos
105: nutsy
106: unwit
107: stoup
108: blist
109: swopt
110: zizit
111: touzy
112: tippy
113: topoi
114: goths
115: tushy
116: timbo
117: doubt
118: unfit
119: stoop
120: situs
121: thymi
122: shist
123: butoh
124: lotos
125: joust
126: stony
127: spoot
128: soths
129: uplit
130: potin
131: tulip
132: toffy
133: smowt
134: tondi
135: ymolt
136: piths
137: potoo
138: bothy
139: might
140: piton
141: thigh
142: shoot
143: swift
144: tolus
145: bundt
146: wisht
147: thous
148: tondo
149: stood
150: withs
151: oobit
152: tousy
153: tiyin
154: kotos
155: pilot
156: thuds
157: towny
158: noint
159: toing
160: timid
161: stopt
162: think
163: styli
164: boult
165: biont
166: butut
167: stond
168: tusky
169: tinny
170: tings
171: nitid
172: towzy
173: stulm
174: nutso
175: thong
176: tymps
177: thowl
178: idiot
179: tophi
180: tying
181: mythi
182: immit
183: untin
184: odist
185: fixit
186: tolls
187: ghost
188: stomp
189: stimy
190: joint
191: thing
192: until
193: toppy
194: tigon
195: tools
196: unlit
197: joist
198: thiol
199: quist
200: stobs
201: toddy
202: towsy
203: ditsy
204: twiny
205: tolyl
206: snift
207: myths
208: towns
209: toffs
210: stonk
211: motif
212: boost
213: quoit
214: tiddy
215: typos
216: gutsy
217: whipt
218: bitou
219: outgo
220: digit
221: bhoot
222: kight
223: titup
224: stong
225: mothy
226: tonus
227: shtum
228: bight
229: outdo
230: thumb
231: kitul
232: sotol
233: flout
234: skint
235: tophs
236: ditzy
237: hoist
238: tupik
239: built
240: timps
241: moths
242: impot
243: pinot
244: futon
245: study
246: pipit
247: stogy
248: hight
249: oubit
250: stums
251: stink
252: muton
253: ottos
254: tough
255: shunt
256: stonn
257: fluyt
258: butyl
259: whist
260: ootid
261: stows
262: jigot
263: tsubo
264: tonks
265: blunt
266: nooit
267: tipsy
268: whift
269: ovist
270: niton
271: inwit
272: kotow
273: visit
274: toons
275: posit
276: fitly
277: outby
278: typps
279: tungs
280: tiyns
281: touks
282: tulsi
283: poynt
284: fount
285: mythy
286: inust
287: tilly
288: snoot
289: tills
290: glift
291: sight
292: twink
293: pluot
294: glint
295: stoun
296: suint
297: tumid
298: twixt
299: bigot
300: motus
301: tyiyn
302: stumm
303: pithy
304: titis
305: stint
306: binit
307: stubs
308: shout
309: kiths
310: snout
311: tinks
312: tinds
313: stoss
314: spilt
315: twilt
316: pivot
317: stops
318: stump
319: topos
320: kutus
321: stilb
322: sluit
323: whoot
324: thunk
325: stilt
326: zoist
327: lotus
328: dixit
329: input
330: thilk
331: tiffs
332: stunt
333: ought
334: tooms
335: midst
336: wight
337: stymy
338: shift
339: bitos
340: touns
341: twist
342: gosht
343: stung
344: tokos
345: puton
346: twins
347: stuff
348: light
349: thugs
350: stuns
351: liths
352: didst
353: stivy
354: notum
355: ungot
356: quint
357: muist
358: tuism
359: stylo
360: tipis
361: twigs
362: pitot
363: musit
364: thump
365: divot
366: hotly
367: thigs
368: stout
369: motis
```
</details>

Let's say we then choose **STUMP** from this long list: 

![second wordle guess](https://i.imgur.com/NGTAt4j.png)

Now we're getting somewhere! We would then add ```-2 'Stu??'``` to our ```churlish``` command, and we can add ```MP``` to ```--wrong```. 

```churlish -1 '???t?' -2 'Stu??' -w craemp```

```churlish``` has now narrowed our valid guesses down to 4: 

```
1: situs
2: suint
3: shout
4: snout
```

Time to get lucky. Let's say we guess **SHOUT**: 

![third wordle guess](https://i.imgur.com/qGaFwUc.png)

STUPID WORDLE. However, ```churlish``` does not care about our pain. Our third pattern would then be ```-3 'S?OUT'```, with wrong letters ```H``` to add: 

```churlish -1 '???t?' -2 '?tu??' -3 'u??iT' -w craemph```

```churlish``` now gives us the only remaining answer: 
```
1: snout
```
Try it for yourself and see! 