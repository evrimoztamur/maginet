# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 246 / 246 matchups complete, 7380 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-b68d3374820e3a28`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Basics I

Map (0, 0); distance 1.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% |
| Hard | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 30/0/0/0; mean 4.3 plies (3–15); 2283 nodes; node saturation 0.0%; fallbacks 0/128.

- Easy/Normal: W/L/D/U 30/0/0/0; mean 3.8 plies (3–13); 4806 nodes; node saturation 0.0%; fallbacks 0/114.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 4.1 plies (3–15); 19325 nodes; node saturation 0.0%; fallbacks 0/122.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 3.3 plies (3–9); 6811 nodes; node saturation 0.0%; fallbacks 0/98.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 3.0 plies (3–3); 8595 nodes; node saturation 0.0%; fallbacks 0/90.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 3.0 plies (3–3); 18048 nodes; node saturation 0.0%; fallbacks 0/90.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 6.2 plies (3–15); 43448 nodes; node saturation 0.0%; fallbacks 0/186.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 5.2 plies (3–15); 45900 nodes; node saturation 0.0%; fallbacks 0/156.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 5.5 plies (3–15); 65810 nodes; node saturation 0.0%; fallbacks 0/164.

## Basics II

Map (1, 0); distance 2.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 56.7% [39.2, 72.6]; U=0; range 56.7–56.7% | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% |
| Normal | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% |
| Hard | 83.3% [66.4, 92.7]; U=0; range 83.3–83.3% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 17/13/0/0; mean 6.6 plies (3–18); 4569 nodes; node saturation 0.0%; fallbacks 0/197.

- Easy/Normal: W/L/D/U 15/15/0/0; mean 4.7 plies (3–9); 12993 nodes; node saturation 0.0%; fallbacks 0/141.

- Easy/Hard: W/L/D/U 15/15/0/0; mean 6.0 plies (3–15); 96382 nodes; node saturation 0.0%; fallbacks 0/181.

- Normal/Easy: W/L/D/U 22/8/0/0; mean 4.3 plies (3–10); 17407 nodes; node saturation 0.0%; fallbacks 0/128.

- Normal/Normal: W/L/D/U 20/10/0/0; mean 4.1 plies (3–11); 23418 nodes; node saturation 0.0%; fallbacks 0/124.

- Normal/Hard: W/L/D/U 20/10/0/0; mean 4.8 plies (3–18); 106766 nodes; node saturation 0.0%; fallbacks 0/144.

- Hard/Easy: W/L/D/U 25/5/0/0; mean 5.6 plies (3–17); 231463 nodes; node saturation 0.0%; fallbacks 0/167.

- Hard/Normal: W/L/D/U 23/7/0/0; mean 5.2 plies (3–18); 235016 nodes; node saturation 0.0%; fallbacks 0/155.

- Hard/Hard: W/L/D/U 23/7/0/0; mean 5.1 plies (3–18); 289490 nodes; node saturation 0.0%; fallbacks 0/152.

## Basics III

Map (2, 0); distance 3.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% |
| Normal | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; U=0; range 86.7–86.7% | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% |
| Hard | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 20/8/2/0; mean 14.8 plies (5–25); 13559 nodes; node saturation 0.0%; fallbacks 0/443.

- Easy/Normal: W/L/D/U 16/12/2/0; mean 14.2 plies (5–22); 63496 nodes; node saturation 0.0%; fallbacks 0/425.

- Easy/Hard: W/L/D/U 16/13/1/0; mean 8.8 plies (5–22); 697203 nodes; node saturation 0.8%; fallbacks 0/263.

- Normal/Easy: W/L/D/U 28/1/1/0; mean 10.8 plies (5–21); 75211 nodes; node saturation 0.0%; fallbacks 0/323.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 10.0 plies (5–22); 115942 nodes; node saturation 0.0%; fallbacks 0/300.

- Normal/Hard: W/L/D/U 22/8/0/0; mean 7.8 plies (5–22); 769574 nodes; node saturation 1.3%; fallbacks 0/235.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 15.2 plies (5–22); 1123397 nodes; node saturation 7.4%; fallbacks 0/457.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 18.5 plies (5–22); 1377367 nodes; node saturation 6.7%; fallbacks 0/554.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 12.6 plies (5–36); 2327706 nodes; node saturation 10.8%; fallbacks 0/379.

## Basics IV

Map (2, -1); distance 4.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Normal | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% | 40.0% [24.6, 57.7]; U=0; range 40.0–40.0% | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% |
| Hard | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 80.0% [62.7, 90.5]; U=0; range 80.0–80.0% | 83.3% [66.4, 92.7]; U=0; range 83.3–83.3% |

- Easy/Easy: W/L/D/U 5/20/5/0; mean 17.0 plies (6–26); 9396 nodes; node saturation 0.0%; fallbacks 0/510.

- Easy/Normal: W/L/D/U 4/19/7/0; mean 13.5 plies (6–22); 37137 nodes; node saturation 0.0%; fallbacks 0/405.

- Easy/Hard: W/L/D/U 3/25/2/0; mean 15.1 plies (6–26); 480646 nodes; node saturation 0.0%; fallbacks 0/452.

- Normal/Easy: W/L/D/U 20/8/2/0; mean 15.2 plies (6–26); 52137 nodes; node saturation 0.0%; fallbacks 0/457.

- Normal/Normal: W/L/D/U 12/18/0/0; mean 14.7 plies (6–21); 82748 nodes; node saturation 0.0%; fallbacks 0/441.

- Normal/Hard: W/L/D/U 7/22/1/0; mean 16.1 plies (7–26); 548454 nodes; node saturation 0.0%; fallbacks 0/484.

- Hard/Easy: W/L/D/U 27/2/1/0; mean 11.8 plies (7–23); 697812 nodes; node saturation 0.0%; fallbacks 0/354.

- Hard/Normal: W/L/D/U 24/4/2/0; mean 12.2 plies (8–27); 731388 nodes; node saturation 0.0%; fallbacks 0/367.

- Hard/Hard: W/L/D/U 25/4/1/0; mean 12.9 plies (7–33); 1319882 nodes; node saturation 0.0%; fallbacks 0/386.

## Patterns I

Map (3, -1); distance 5.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% | 26.7% [14.2, 44.4]; U=0; range 26.7–26.7% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Hard | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% |

- Easy/Easy: W/L/D/U 11/18/1/0; mean 31.2 plies (20–55); 59418 nodes; node saturation 0.0%; fallbacks 0/937.

- Easy/Normal: W/L/D/U 1/26/3/0; mean 28.2 plies (18–40); 503366 nodes; node saturation 0.0%; fallbacks 0/846.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 28.5 plies (20–38); 6252263 nodes; node saturation 29.8%; fallbacks 0/856.

- Normal/Easy: W/L/D/U 20/8/2/0; mean 30.9 plies (19–41); 513088 nodes; node saturation 0.0%; fallbacks 0/928.

- Normal/Normal: W/L/D/U 8/20/2/0; mean 30.0 plies (19–55); 918408 nodes; node saturation 0.0%; fallbacks 0/901.

- Normal/Hard: W/L/D/U 3/25/2/0; mean 30.8 plies (20–45); 6691603 nodes; node saturation 28.0%; fallbacks 0/923.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 31.8 plies (19–44); 7231571 nodes; node saturation 32.1%; fallbacks 0/955.

- Hard/Normal: W/L/D/U 23/5/2/0; mean 35.0 plies (27–48); 8308584 nodes; node saturation 31.3%; fallbacks 0/1051.

- Hard/Hard: W/L/D/U 13/16/1/0; mean 34.9 plies (24–49); 14375500 nodes; node saturation 56.6%; fallbacks 0/1048.

## Patterns II

Map (4, -1); distance 6.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Normal | 70.0% [52.1, 83.3]; U=0; range 70.0–70.0% | 60.0% [42.3, 75.4]; U=0; range 60.0–60.0% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Hard | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% |

- Easy/Easy: W/L/D/U 19/8/3/0; mean 33.2 plies (21–53); 33163 nodes; node saturation 0.0%; fallbacks 0/996.

- Easy/Normal: W/L/D/U 2/21/7/0; mean 31.3 plies (21–48); 213419 nodes; node saturation 0.0%; fallbacks 0/940.

- Easy/Hard: W/L/D/U 1/25/4/0; mean 26.2 plies (21–36); 4763608 nodes; node saturation 7.4%; fallbacks 0/786.

- Normal/Easy: W/L/D/U 21/0/9/0; mean 30.1 plies (21–52); 186549 nodes; node saturation 0.0%; fallbacks 0/902.

- Normal/Normal: W/L/D/U 18/8/4/0; mean 35.0 plies (21–52); 393928 nodes; node saturation 0.0%; fallbacks 0/1051.

- Normal/Hard: W/L/D/U 1/20/9/0; mean 26.7 plies (21–51); 4681956 nodes; node saturation 3.9%; fallbacks 0/802.

- Hard/Easy: W/L/D/U 28/0/2/0; mean 27.5 plies (21–53); 4752666 nodes; node saturation 3.2%; fallbacks 0/825.

- Hard/Normal: W/L/D/U 20/2/8/0; mean 25.7 plies (21–48); 4409132 nodes; node saturation 1.3%; fallbacks 0/772.

- Hard/Hard: W/L/D/U 4/1/25/0; mean 25.1 plies (21–49); 7953859 nodes; node saturation 3.3%; fallbacks 0/754.

## Patterns III

Map (5, -1); distance 7.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; U=0; range 46.7–46.7% | 30.0% [16.7, 47.9]; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Normal | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% | 70.0% [52.1, 83.3]; U=0; range 70.0–70.0% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% |
| Hard | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | 80.0% [62.7, 90.5]; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 14/11/5/0; mean 34.8 plies (27–52); 78914 nodes; node saturation 0.0%; fallbacks 0/1045.

- Easy/Normal: W/L/D/U 9/16/5/0; mean 38.0 plies (27–58); 776057 nodes; node saturation 0.0%; fallbacks 0/1141.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 35.6 plies (27–52); 7866444 nodes; node saturation 32.2%; fallbacks 0/1068.

- Normal/Easy: W/L/D/U 22/6/2/0; mean 33.1 plies (27–48); 801496 nodes; node saturation 0.0%; fallbacks 0/992.

- Normal/Normal: W/L/D/U 21/3/6/0; mean 37.0 plies (25–55); 1627492 nodes; node saturation 0.0%; fallbacks 0/1111.

- Normal/Hard: W/L/D/U 11/16/3/0; mean 34.3 plies (21–57); 8466210 nodes; node saturation 32.7%; fallbacks 0/1029.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 33.8 plies (27–47); 8318800 nodes; node saturation 36.2%; fallbacks 0/1014.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 34.1 plies (27–46); 8885319 nodes; node saturation 35.3%; fallbacks 0/1022.

- Hard/Hard: W/L/D/U 24/5/1/0; mean 33.9 plies (27–50); 15188279 nodes; node saturation 63.1%; fallbacks 0/1018.

## Diagonals I

Map (4, 0); distance 7.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; U=0; range 46.7–46.7% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Normal | 60.0% [42.3, 75.4]; U=0; range 60.0–60.0% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% |
| Hard | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 20.0% [9.5, 37.3]; U=0; range 20.0–20.0% |

- Easy/Easy: W/L/D/U 14/14/2/0; mean 16.5 plies (10–25); 8964 nodes; node saturation 0.0%; fallbacks 0/494.

- Easy/Normal: W/L/D/U 3/27/0/0; mean 15.1 plies (9–26); 40121 nodes; node saturation 0.0%; fallbacks 0/453.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 16.8 plies (8–30); 556981 nodes; node saturation 0.0%; fallbacks 0/503.

- Normal/Easy: W/L/D/U 18/10/2/0; mean 16.9 plies (9–29); 47061 nodes; node saturation 0.0%; fallbacks 0/506.

- Normal/Normal: W/L/D/U 11/15/4/0; mean 17.8 plies (11–29); 87618 nodes; node saturation 0.0%; fallbacks 0/533.

- Normal/Hard: W/L/D/U 5/20/5/0; mean 18.0 plies (12–26); 681264 nodes; node saturation 0.0%; fallbacks 0/540.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 16.1 plies (11–25); 471576 nodes; node saturation 0.0%; fallbacks 0/484.

- Hard/Normal: W/L/D/U 16/8/6/0; mean 17.8 plies (11–26); 745581 nodes; node saturation 0.0%; fallbacks 0/534.

- Hard/Hard: W/L/D/U 6/9/15/0; mean 17.0 plies (14–26); 1319212 nodes; node saturation 0.0%; fallbacks 0/510.

## Diagonals II

Map (4, 1); distance 8.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; U=0; range 46.7–46.7% | 20.0% [9.5, 37.3]; U=0; range 20.0–20.0% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Hard | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | 80.0% [62.7, 90.5]; U=0; range 80.0–80.0% | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% |

- Easy/Easy: W/L/D/U 14/14/2/0; mean 30.7 plies (21–46); 44983 nodes; node saturation 0.0%; fallbacks 0/921.

- Easy/Normal: W/L/D/U 6/19/5/0; mean 29.2 plies (21–45); 299469 nodes; node saturation 0.0%; fallbacks 0/877.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 29.1 plies (18–44); 5887249 nodes; node saturation 25.0%; fallbacks 0/873.

- Normal/Easy: W/L/D/U 23/4/3/0; mean 29.4 plies (21–52); 329642 nodes; node saturation 0.0%; fallbacks 0/882.

- Normal/Normal: W/L/D/U 13/12/5/0; mean 33.0 plies (23–48); 637802 nodes; node saturation 0.0%; fallbacks 0/991.

- Normal/Hard: W/L/D/U 2/28/0/0; mean 30.5 plies (21–54); 6222122 nodes; node saturation 23.9%; fallbacks 0/914.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 28.0 plies (21–43); 6167692 nodes; node saturation 29.3%; fallbacks 0/839.

- Hard/Normal: W/L/D/U 24/4/2/0; mean 27.2 plies (21–43); 5859377 nodes; node saturation 25.9%; fallbacks 0/817.

- Hard/Hard: W/L/D/U 22/8/0/0; mean 29.2 plies (21–46); 11388262 nodes; node saturation 47.8%; fallbacks 0/877.

## Diagonals III

Map (4, 2); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Normal | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 46.7% [30.2, 63.9]; U=0; range 46.7–46.7% | 33.3% [19.2, 51.2]; U=0; range 33.3–33.3% |
| Hard | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 80.0% [62.7, 90.5]; U=0; range 80.0–80.0% | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% |

- Easy/Easy: W/L/D/U 16/12/2/0; mean 35.7 plies (27–54); 76015 nodes; node saturation 0.0%; fallbacks 0/1071.

- Easy/Normal: W/L/D/U 5/23/2/0; mean 33.2 plies (27–43); 632285 nodes; node saturation 0.0%; fallbacks 0/996.

- Easy/Hard: W/L/D/U 2/26/2/0; mean 33.1 plies (27–54); 7570322 nodes; node saturation 33.0%; fallbacks 0/994.

- Normal/Easy: W/L/D/U 23/7/0/0; mean 35.9 plies (25–64); 663865 nodes; node saturation 0.0%; fallbacks 0/1078.

- Normal/Normal: W/L/D/U 14/12/4/0; mean 33.5 plies (21–50); 1087055 nodes; node saturation 0.0%; fallbacks 0/1004.

- Normal/Hard: W/L/D/U 10/16/4/0; mean 33.4 plies (27–48); 7701378 nodes; node saturation 30.0%; fallbacks 0/1001.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 34.4 plies (27–51); 7982875 nodes; node saturation 33.6%; fallbacks 0/1032.

- Hard/Normal: W/L/D/U 24/4/2/0; mean 30.9 plies (25–50); 7789608 nodes; node saturation 35.1%; fallbacks 0/927.

- Hard/Hard: W/L/D/U 19/9/2/0; mean 33.5 plies (21–51); 14097896 nodes; node saturation 60.4%; fallbacks 0/1006.

## Diagonals IV

Map (4, 3); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; U=0; range 70.0–70.0% | 33.3% [19.2, 51.2]; U=0; range 33.3–33.3% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Hard | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 70.0% [52.1, 83.3]; U=0; range 70.0–70.0% | 46.7% [30.2, 63.9]; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 15/13/2/0; mean 39.5 plies (27–52); 82805 nodes; node saturation 0.0%; fallbacks 0/1186.

- Easy/Normal: W/L/D/U 5/18/7/0; mean 38.5 plies (27–58); 711729 nodes; node saturation 0.0%; fallbacks 0/1155.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 34.9 plies (27–57); 9160755 nodes; node saturation 40.1%; fallbacks 0/1047.

- Normal/Easy: W/L/D/U 21/4/5/0; mean 36.5 plies (27–61); 764530 nodes; node saturation 0.0%; fallbacks 0/1095.

- Normal/Normal: W/L/D/U 10/11/9/0; mean 38.0 plies (27–61); 1468876 nodes; node saturation 0.0%; fallbacks 0/1139.

- Normal/Hard: W/L/D/U 3/19/8/0; mean 36.7 plies (27–64); 9716225 nodes; node saturation 36.9%; fallbacks 0/1102.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 34.7 plies (27–51); 9212002 nodes; node saturation 39.2%; fallbacks 0/1040.

- Hard/Normal: W/L/D/U 21/5/4/0; mean 35.8 plies (27–50); 9609660 nodes; node saturation 38.1%; fallbacks 0/1075.

- Hard/Hard: W/L/D/U 14/11/5/0; mean 38.8 plies (27–51); 19308120 nodes; node saturation 74.8%; fallbacks 0/1163.

## Beams I

Map (5, -2); distance 8.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 56.7% [39.2, 72.6]; U=0; range 56.7–56.7% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% |
| Hard | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% | 30.0% [16.7, 47.9]; U=0; range 30.0–30.0% |

- Easy/Easy: W/L/D/U 17/12/1/0; mean 32.2 plies (21–46); 26341 nodes; node saturation 0.0%; fallbacks 0/965.

- Easy/Normal: W/L/D/U 2/26/2/0; mean 28.6 plies (20–41); 137023 nodes; node saturation 0.0%; fallbacks 0/857.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 30.2 plies (22–52); 2875896 nodes; node saturation 0.0%; fallbacks 0/906.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 29.0 plies (15–47); 137519 nodes; node saturation 0.0%; fallbacks 0/870.

- Normal/Normal: W/L/D/U 13/13/4/0; mean 32.5 plies (18–46); 267762 nodes; node saturation 0.0%; fallbacks 0/974.

- Normal/Hard: W/L/D/U 7/22/1/0; mean 34.5 plies (22–48); 3472899 nodes; node saturation 0.0%; fallbacks 0/1036.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 30.6 plies (19–47); 2886062 nodes; node saturation 0.3%; fallbacks 0/919.

- Hard/Normal: W/L/D/U 22/6/2/0; mean 32.5 plies (23–48); 3050416 nodes; node saturation 0.0%; fallbacks 0/974.

- Hard/Hard: W/L/D/U 9/19/2/0; mean 32.1 plies (22–56); 5479506 nodes; node saturation 0.1%; fallbacks 0/962.

## Beams II

Map (5, -3); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; U=0; range 70.0–70.0% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% |
| Hard | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | 70.0% [52.1, 83.3]; U=0; range 70.0–70.0% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% |

- Easy/Easy: W/L/D/U 7/17/6/0; mean 38.1 plies (27–69); 57720 nodes; node saturation 0.0%; fallbacks 0/1144.

- Easy/Normal: W/L/D/U 5/22/3/0; mean 36.7 plies (27–58); 415716 nodes; node saturation 0.0%; fallbacks 0/1101.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 33.8 plies (24–52); 7491622 nodes; node saturation 29.5%; fallbacks 0/1013.

- Normal/Easy: W/L/D/U 21/5/4/0; mean 40.9 plies (21–63); 480188 nodes; node saturation 0.0%; fallbacks 0/1228.

- Normal/Normal: W/L/D/U 11/11/8/0; mean 40.2 plies (27–58); 858508 nodes; node saturation 0.0%; fallbacks 0/1206.

- Normal/Hard: W/L/D/U 5/18/7/0; mean 37.2 plies (27–54); 8430176 nodes; node saturation 28.3%; fallbacks 0/1115.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 35.8 plies (27–53); 7973341 nodes; node saturation 29.9%; fallbacks 0/1073.

- Hard/Normal: W/L/D/U 21/3/6/0; mean 37.1 plies (27–72); 8694212 nodes; node saturation 30.4%; fallbacks 0/1112.

- Hard/Hard: W/L/D/U 13/9/8/0; mean 35.3 plies (27–68); 14707239 nodes; node saturation 51.1%; fallbacks 0/1058.

## Beams III

Map (6, -3); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 56.7% [39.2, 72.6]; U=0; range 56.7–56.7% | 40.0% [24.6, 57.7]; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Hard | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% |

- Easy/Easy: W/L/D/U 16/11/3/0; mean 28.4 plies (20–43); 57280 nodes; node saturation 0.0%; fallbacks 0/852.

- Easy/Normal: W/L/D/U 5/24/1/0; mean 25.0 plies (18–39); 362933 nodes; node saturation 0.0%; fallbacks 0/751.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 25.9 plies (18–38); 6461986 nodes; node saturation 35.7%; fallbacks 0/778.

- Normal/Easy: W/L/D/U 17/10/3/0; mean 28.6 plies (18–41); 526553 nodes; node saturation 0.0%; fallbacks 0/857.

- Normal/Normal: W/L/D/U 12/15/3/0; mean 27.3 plies (17–39); 776398 nodes; node saturation 0.0%; fallbacks 0/820.

- Normal/Hard: W/L/D/U 2/25/3/0; mean 28.2 plies (21–38); 7123026 nodes; node saturation 34.1%; fallbacks 0/847.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 29.3 plies (19–43); 7326126 nodes; node saturation 35.9%; fallbacks 0/879.

- Hard/Normal: W/L/D/U 28/1/1/0; mean 27.5 plies (17–35); 7065521 nodes; node saturation 35.4%; fallbacks 0/824.

- Hard/Hard: W/L/D/U 16/12/2/0; mean 28.6 plies (19–42); 13950633 nodes; node saturation 72.3%; fallbacks 0/859.

## Shields I

Map (5, 2); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Normal | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Hard | 26.7% [14.2, 44.4]; U=0; range 26.7–26.7% | 33.3% [19.2, 51.2]; U=0; range 33.3–33.3% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |

- Easy/Easy: W/L/D/U 3/14/13/0; mean 14.7 plies (8–23); 5957 nodes; node saturation 0.0%; fallbacks 0/441.

- Easy/Normal: W/L/D/U 2/16/12/0; mean 13.1 plies (8–22); 17225 nodes; node saturation 0.0%; fallbacks 0/392.

- Easy/Hard: W/L/D/U 1/16/13/0; mean 14.5 plies (8–26); 156034 nodes; node saturation 0.0%; fallbacks 0/435.

- Normal/Easy: W/L/D/U 1/4/25/0; mean 13.6 plies (8–18); 20063 nodes; node saturation 0.0%; fallbacks 0/408.

- Normal/Normal: W/L/D/U 0/4/26/0; mean 13.4 plies (8–24); 35061 nodes; node saturation 0.0%; fallbacks 0/403.

- Normal/Hard: W/L/D/U 1/5/24/0; mean 13.8 plies (8–26); 169799 nodes; node saturation 0.0%; fallbacks 0/415.

- Hard/Easy: W/L/D/U 8/3/19/0; mean 16.0 plies (8–26); 225016 nodes; node saturation 0.0%; fallbacks 0/481.

- Hard/Normal: W/L/D/U 10/2/18/0; mean 14.4 plies (8–21); 266176 nodes; node saturation 0.0%; fallbacks 0/433.

- Hard/Hard: W/L/D/U 2/4/24/0; mean 14.2 plies (8–27); 330667 nodes; node saturation 0.0%; fallbacks 0/427.

## Shields II

Map (6, 2); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Normal | 30.0% [16.7, 47.9]; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Hard | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% |

- Easy/Easy: W/L/D/U 3/25/2/0; mean 24.0 plies (18–38); 35572 nodes; node saturation 0.0%; fallbacks 0/720.

- Easy/Normal: W/L/D/U 4/26/0/0; mean 23.5 plies (16–35); 244854 nodes; node saturation 0.0%; fallbacks 0/705.

- Easy/Hard: W/L/D/U 1/29/0/0; mean 25.7 plies (18–36); 5424473 nodes; node saturation 28.0%; fallbacks 0/771.

- Normal/Easy: W/L/D/U 9/19/2/0; mean 26.7 plies (18–38); 238786 nodes; node saturation 0.0%; fallbacks 0/800.

- Normal/Normal: W/L/D/U 5/23/2/0; mean 25.2 plies (16–41); 470178 nodes; node saturation 0.0%; fallbacks 0/756.

- Normal/Hard: W/L/D/U 1/29/0/0; mean 26.4 plies (18–38); 5422745 nodes; node saturation 23.7%; fallbacks 0/792.

- Hard/Easy: W/L/D/U 16/12/2/0; mean 28.1 plies (16–42); 5915664 nodes; node saturation 28.5%; fallbacks 0/842.

- Hard/Normal: W/L/D/U 13/16/1/0; mean 26.8 plies (16–37); 5921883 nodes; node saturation 29.0%; fallbacks 0/803.

- Hard/Hard: W/L/D/U 4/24/2/0; mean 26.7 plies (16–37); 11076090 nodes; node saturation 57.6%; fallbacks 0/800.

## Shields III

Map (7, 2); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 30.0% [16.7, 47.9]; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Normal | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% |
| Hard | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; U=0; range 80.0–80.0% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 21.3 plies (13–32); 24941 nodes; node saturation 0.0%; fallbacks 0/638.

- Easy/Normal: W/L/D/U 9/20/1/0; mean 20.8 plies (12–36); 163897 nodes; node saturation 0.0%; fallbacks 0/625.

- Easy/Hard: W/L/D/U 3/26/1/0; mean 20.3 plies (11–32); 3148670 nodes; node saturation 18.7%; fallbacks 0/609.

- Normal/Easy: W/L/D/U 19/4/7/0; mean 19.6 plies (13–31); 155471 nodes; node saturation 0.0%; fallbacks 0/588.

- Normal/Normal: W/L/D/U 16/12/2/0; mean 20.2 plies (11–32); 285831 nodes; node saturation 0.0%; fallbacks 0/607.

- Normal/Hard: W/L/D/U 7/20/3/0; mean 19.9 plies (15–32); 3039960 nodes; node saturation 17.1%; fallbacks 0/597.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 19.2 plies (11–31); 3486984 nodes; node saturation 23.6%; fallbacks 0/576.

- Hard/Normal: W/L/D/U 24/2/4/0; mean 20.3 plies (13–45); 3852102 nodes; node saturation 24.2%; fallbacks 0/608.

- Hard/Hard: W/L/D/U 23/6/1/0; mean 16.3 plies (11–28); 5476523 nodes; node saturation 41.1%; fallbacks 0/489.

## Challenge I

Map (2, 1); distance 4.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Hard | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 80.0% [62.7, 90.5]; U=0; range 80.0–80.0% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% |

- Easy/Easy: W/L/D/U 13/17/0/0; mean 16.3 plies (12–25); 20311 nodes; node saturation 0.0%; fallbacks 0/488.

- Easy/Normal: W/L/D/U 5/24/1/0; mean 17.2 plies (12–28); 139939 nodes; node saturation 0.0%; fallbacks 0/517.

- Easy/Hard: W/L/D/U 0/27/3/0; mean 19.2 plies (11–29); 2897771 nodes; node saturation 18.5%; fallbacks 0/577.

- Normal/Easy: W/L/D/U 19/10/1/0; mean 18.0 plies (11–34); 138481 nodes; node saturation 0.0%; fallbacks 0/539.

- Normal/Normal: W/L/D/U 11/10/9/0; mean 17.9 plies (12–25); 273278 nodes; node saturation 0.0%; fallbacks 0/537.

- Normal/Hard: W/L/D/U 3/24/3/0; mean 17.8 plies (12–29); 3114944 nodes; node saturation 21.7%; fallbacks 0/535.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 18.9 plies (13–25); 3052652 nodes; node saturation 20.3%; fallbacks 0/566.

- Hard/Normal: W/L/D/U 24/3/3/0; mean 19.7 plies (11–29); 3427223 nodes; node saturation 23.3%; fallbacks 0/592.

- Hard/Hard: W/L/D/U 16/8/6/0; mean 18.4 plies (12–31); 6333095 nodes; node saturation 45.8%; fallbacks 0/552.

## Challenge II

Map (6, -1); distance 8.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 30.0% [16.7, 47.9]; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Normal | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% | 33.3% [19.2, 51.2]; U=0; range 33.3–33.3% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% |
| Hard | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% | 30.0% [16.7, 47.9]; U=0; range 30.0–30.0% |

- Easy/Easy: W/L/D/U 9/13/8/0; mean 27.4 plies (16–40); 27493 nodes; node saturation 0.0%; fallbacks 0/821.

- Easy/Normal: W/L/D/U 5/18/7/0; mean 23.7 plies (13–39); 160149 nodes; node saturation 0.0%; fallbacks 0/711.

- Easy/Hard: W/L/D/U 1/25/4/0; mean 21.1 plies (12–37); 3409841 nodes; node saturation 18.0%; fallbacks 0/634.

- Normal/Easy: W/L/D/U 19/6/5/0; mean 21.3 plies (11–29); 147285 nodes; node saturation 0.0%; fallbacks 0/639.

- Normal/Normal: W/L/D/U 10/11/9/0; mean 20.4 plies (10–39); 250190 nodes; node saturation 0.0%; fallbacks 0/612.

- Normal/Hard: W/L/D/U 4/21/5/0; mean 20.1 plies (12–28); 3392591 nodes; node saturation 19.8%; fallbacks 0/602.

- Hard/Easy: W/L/D/U 23/2/5/0; mean 22.1 plies (15–37); 3509265 nodes; node saturation 18.7%; fallbacks 0/663.

- Hard/Normal: W/L/D/U 22/4/4/0; mean 21.9 plies (13–29); 3086961 nodes; node saturation 17.8%; fallbacks 0/657.

- Hard/Hard: W/L/D/U 9/18/3/0; mean 23.5 plies (14–39); 7553762 nodes; node saturation 40.4%; fallbacks 0/706.

## Challenge III

Map (7, -3); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Hard | 86.7% [70.3, 94.7]; U=0; range 86.7–86.7% | 40.0% [24.6, 57.7]; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |

- Easy/Easy: W/L/D/U 11/15/4/0; mean 45.5 plies (27–79); 104276 nodes; node saturation 0.0%; fallbacks 0/1366.

- Easy/Normal: W/L/D/U 0/30/0/0; mean 35.4 plies (27–51); 816203 nodes; node saturation 0.1%; fallbacks 0/1061.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 34.8 plies (27–58); 9194387 nodes; node saturation 39.8%; fallbacks 0/1045.

- Normal/Easy: W/L/D/U 23/4/3/0; mean 39.2 plies (27–59); 917911 nodes; node saturation 0.0%; fallbacks 0/1175.

- Normal/Normal: W/L/D/U 7/14/9/0; mean 40.7 plies (27–62); 1674094 nodes; node saturation 0.0%; fallbacks 0/1221.

- Normal/Hard: W/L/D/U 0/27/3/0; mean 34.9 plies (27–64); 9744268 nodes; node saturation 39.2%; fallbacks 0/1047.

- Hard/Easy: W/L/D/U 26/2/2/0; mean 39.0 plies (27–57); 10563667 nodes; node saturation 41.6%; fallbacks 0/1170.

- Hard/Normal: W/L/D/U 12/10/8/0; mean 40.4 plies (27–60); 11085182 nodes; node saturation 38.7%; fallbacks 0/1211.

- Hard/Hard: W/L/D/U 2/27/1/0; mean 37.2 plies (27–49); 18719026 nodes; node saturation 75.6%; fallbacks 0/1117.

## Challenge IV

Map (7, 1); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; U=0; range 10.0–10.0% |
| Hard | 83.3% [66.4, 92.7]; U=0; range 83.3–83.3% | 60.0% [42.3, 75.4]; U=0; range 60.0–60.0% | 33.3% [19.2, 51.2]; U=0; range 33.3–33.3% |

- Easy/Easy: W/L/D/U 11/15/4/0; mean 41.0 plies (27–69); 83998 nodes; node saturation 0.0%; fallbacks 0/1229.

- Easy/Normal: W/L/D/U 3/25/2/0; mean 45.1 plies (28–62); 825844 nodes; node saturation 0.0%; fallbacks 0/1354.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 41.0 plies (27–60); 10927498 nodes; node saturation 38.7%; fallbacks 0/1230.

- Normal/Easy: W/L/D/U 16/4/10/0; mean 40.6 plies (27–55); 722790 nodes; node saturation 0.0%; fallbacks 0/1218.

- Normal/Normal: W/L/D/U 11/10/9/0; mean 42.4 plies (27–60); 1594545 nodes; node saturation 0.0%; fallbacks 0/1271.

- Normal/Hard: W/L/D/U 3/22/5/0; mean 40.6 plies (27–60); 11446862 nodes; node saturation 38.3%; fallbacks 0/1218.

- Hard/Easy: W/L/D/U 25/2/3/0; mean 37.6 plies (27–60); 10015050 nodes; node saturation 38.9%; fallbacks 0/1128.

- Hard/Normal: W/L/D/U 18/5/7/0; mean 38.2 plies (27–52); 10798335 nodes; node saturation 39.0%; fallbacks 0/1145.

- Hard/Hard: W/L/D/U 10/13/7/0; mean 42.2 plies (27–69); 21037666 nodes; node saturation 72.7%; fallbacks 0/1265.

## Rite I

Map (7, -2); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Normal | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 56.7% [39.2, 72.6]; U=0; range 56.7–56.7% | 33.3% [19.2, 51.2]; U=0; range 33.3–33.3% |
| Hard | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% |

- Easy/Easy: W/L/D/U 15/10/5/0; mean 51.5 plies (36–79); 139220 nodes; node saturation 0.0%; fallbacks 0/1546.

- Easy/Normal: W/L/D/U 2/25/3/0; mean 45.4 plies (33–66); 1426106 nodes; node saturation 1.5%; fallbacks 0/1361.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 44.4 plies (33–58); 11811702 nodes; node saturation 40.7%; fallbacks 0/1332.

- Normal/Easy: W/L/D/U 27/1/2/0; mean 43.6 plies (29–61); 1470538 nodes; node saturation 2.1%; fallbacks 0/1309.

- Normal/Normal: W/L/D/U 17/11/2/0; mean 46.3 plies (33–66); 2833197 nodes; node saturation 4.1%; fallbacks 0/1388.

- Normal/Hard: W/L/D/U 10/16/4/0; mean 46.7 plies (33–64); 13310399 nodes; node saturation 41.4%; fallbacks 0/1400.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 41.9 plies (33–62); 10958150 nodes; node saturation 39.1%; fallbacks 0/1257.

- Hard/Normal: W/L/D/U 23/4/3/0; mean 44.9 plies (33–58); 12563325 nodes; node saturation 41.6%; fallbacks 0/1348.

- Hard/Hard: W/L/D/U 15/10/5/0; mean 48.4 plies (29–71); 23069076 nodes; node saturation 71.1%; fallbacks 0/1452.

## Rite II

Map (7, -1); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 40.0% [24.6, 57.7]; U=0; range 40.0–40.0% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Hard | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% | 20.0% [9.5, 37.3]; U=0; range 20.0–20.0% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% |

- Easy/Easy: W/L/D/U 5/3/22/0; mean 26.4 plies (21–48); 51403 nodes; node saturation 0.0%; fallbacks 0/792.

- Easy/Normal: W/L/D/U 2/8/20/0; mean 26.3 plies (21–54); 364559 nodes; node saturation 0.0%; fallbacks 0/789.

- Easy/Hard: W/L/D/U 0/11/19/0; mean 23.1 plies (21–40); 5375225 nodes; node saturation 28.6%; fallbacks 0/693.

- Normal/Easy: W/L/D/U 12/1/17/0; mean 25.7 plies (21–44); 376172 nodes; node saturation 0.0%; fallbacks 0/771.

- Normal/Normal: W/L/D/U 4/7/19/0; mean 28.4 plies (21–50); 719013 nodes; node saturation 0.0%; fallbacks 0/851.

- Normal/Hard: W/L/D/U 0/7/23/0; mean 23.3 plies (21–38); 5323775 nodes; node saturation 24.0%; fallbacks 0/699.

- Hard/Easy: W/L/D/U 19/0/11/0; mean 24.8 plies (21–41); 6076375 nodes; node saturation 31.1%; fallbacks 0/745.

- Hard/Normal: W/L/D/U 6/0/24/0; mean 22.3 plies (21–49); 5162342 nodes; node saturation 26.3%; fallbacks 0/668.

- Hard/Hard: W/L/D/U 5/2/23/0; mean 25.7 plies (21–41); 11113914 nodes; node saturation 52.2%; fallbacks 0/772.

## Rite III

Map (7, 0); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Normal | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% |
| Hard | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% |

- Easy/Easy: W/L/D/U 13/15/2/0; mean 36.8 plies (27–57); 75964 nodes; node saturation 0.0%; fallbacks 0/1103.

- Easy/Normal: W/L/D/U 5/22/3/0; mean 36.1 plies (27–49); 745425 nodes; node saturation 0.0%; fallbacks 0/1083.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 33.4 plies (26–54); 8046993 nodes; node saturation 35.5%; fallbacks 0/1001.

- Normal/Easy: W/L/D/U 23/6/1/0; mean 33.9 plies (25–62); 712346 nodes; node saturation 0.0%; fallbacks 0/1017.

- Normal/Normal: W/L/D/U 13/10/7/0; mean 35.8 plies (23–55); 1428706 nodes; node saturation 0.3%; fallbacks 0/1073.

- Normal/Hard: W/L/D/U 4/22/4/0; mean 34.3 plies (25–52); 8466751 nodes; node saturation 33.4%; fallbacks 0/1030.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 35.7 plies (23–44); 8899118 nodes; node saturation 37.2%; fallbacks 0/1072.

- Hard/Normal: W/L/D/U 23/5/2/0; mean 34.0 plies (25–47); 9186803 nodes; node saturation 37.7%; fallbacks 0/1020.

- Hard/Hard: W/L/D/U 11/14/5/0; mean 36.5 plies (27–53); 17491396 nodes; node saturation 71.3%; fallbacks 0/1096.

## Rite IV

Map (8, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Normal | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; U=0; range 13.3–13.3% |
| Hard | 90.0% [74.4, 96.5]; U=0; range 90.0–90.0% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 36.7% [21.9, 54.5]; U=0; range 36.7–36.7% |

- Easy/Easy: W/L/D/U 13/15/2/0; mean 36.8 plies (27–57); 75964 nodes; node saturation 0.0%; fallbacks 0/1103.

- Easy/Normal: W/L/D/U 5/22/3/0; mean 36.1 plies (27–49); 745425 nodes; node saturation 0.0%; fallbacks 0/1083.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 33.4 plies (26–54); 8046993 nodes; node saturation 35.5%; fallbacks 0/1001.

- Normal/Easy: W/L/D/U 23/6/1/0; mean 33.9 plies (25–62); 712346 nodes; node saturation 0.0%; fallbacks 0/1017.

- Normal/Normal: W/L/D/U 13/10/7/0; mean 35.8 plies (23–55); 1428706 nodes; node saturation 0.3%; fallbacks 0/1073.

- Normal/Hard: W/L/D/U 4/22/4/0; mean 34.3 plies (25–52); 8466751 nodes; node saturation 33.4%; fallbacks 0/1030.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 35.7 plies (23–44); 8899118 nodes; node saturation 37.2%; fallbacks 0/1072.

- Hard/Normal: W/L/D/U 23/5/2/0; mean 34.0 plies (25–47); 9186803 nodes; node saturation 37.7%; fallbacks 0/1020.

- Hard/Hard: W/L/D/U 11/14/5/0; mean 36.5 plies (27–53); 17491396 nodes; node saturation 71.3%; fallbacks 0/1096.

## Ascension I

Map (8, -1); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; U=0; range 0.0–0.0% |
| Normal | 66.7% [48.8, 80.8]; U=0; range 66.7–66.7% | 50.0% [33.2, 66.8]; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; U=0; range 16.7–16.7% |
| Hard | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | 76.7% [59.1, 88.2]; U=0; range 76.7–76.7% | 46.7% [30.2, 63.9]; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 15/10/5/0; mean 42.2 plies (27–61); 109888 nodes; node saturation 0.0%; fallbacks 0/1266.

- Easy/Normal: W/L/D/U 5/20/5/0; mean 40.5 plies (27–59); 985841 nodes; node saturation 0.0%; fallbacks 0/1216.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 37.1 plies (27–52); 10076522 nodes; node saturation 43.0%; fallbacks 0/1112.

- Normal/Easy: W/L/D/U 20/3/7/0; mean 41.7 plies (27–57); 1081758 nodes; node saturation 0.4%; fallbacks 0/1252.

- Normal/Normal: W/L/D/U 15/9/6/0; mean 40.7 plies (27–56); 2069559 nodes; node saturation 1.8%; fallbacks 0/1221.

- Normal/Hard: W/L/D/U 5/24/1/0; mean 39.3 plies (27–56); 11188854 nodes; node saturation 41.3%; fallbacks 0/1178.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 36.9 plies (27–60); 10206692 nodes; node saturation 43.5%; fallbacks 0/1107.

- Hard/Normal: W/L/D/U 23/3/4/0; mean 40.4 plies (27–57); 11464477 nodes; node saturation 40.3%; fallbacks 0/1211.

- Hard/Hard: W/L/D/U 14/12/4/0; mean 41.1 plies (28–55); 21008765 nodes; node saturation 78.2%; fallbacks 0/1232.

## Ascension II

Map (9, -1); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 20.0% [9.5, 37.3]; U=0; range 20.0–20.0% | 26.7% [14.2, 44.4]; U=0; range 26.7–26.7% | 3.3% [0.6, 16.7]; U=0; range 3.3–3.3% |
| Normal | 63.3% [45.5, 78.1]; U=0; range 63.3–63.3% | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% | 6.7% [1.8, 21.3]; U=0; range 6.7–6.7% |
| Hard | 73.3% [55.6, 85.8]; U=0; range 73.3–73.3% | 53.3% [36.1, 69.8]; U=0; range 53.3–53.3% | 23.3% [11.8, 40.9]; U=0; range 23.3–23.3% |

- Easy/Easy: W/L/D/U 6/18/6/0; mean 51.5 plies (33–73); 198542 nodes; node saturation 0.0%; fallbacks 0/1546.

- Easy/Normal: W/L/D/U 8/20/2/0; mean 51.0 plies (33–67); 2148260 nodes; node saturation 5.8%; fallbacks 0/1531.

- Easy/Hard: W/L/D/U 1/28/1/0; mean 47.8 plies (33–74); 13819007 nodes; node saturation 47.0%; fallbacks 0/1433.

- Normal/Easy: W/L/D/U 19/3/8/0; mean 48.2 plies (33–74); 1971502 nodes; node saturation 3.0%; fallbacks 0/1446.

- Normal/Normal: W/L/D/U 7/18/5/0; mean 51.8 plies (33–70); 4017260 nodes; node saturation 10.4%; fallbacks 0/1555.

- Normal/Hard: W/L/D/U 2/25/3/0; mean 50.2 plies (33–94); 15986842 nodes; node saturation 49.4%; fallbacks 0/1507.

- Hard/Easy: W/L/D/U 22/2/6/0; mean 52.5 plies (33–82); 15049373 nodes; node saturation 46.1%; fallbacks 0/1576.

- Hard/Normal: W/L/D/U 16/7/7/0; mean 52.5 plies (34–81); 17160441 nodes; node saturation 50.1%; fallbacks 0/1576.

- Hard/Hard: W/L/D/U 7/21/2/0; mean 53.8 plies (36–86); 29980785 nodes; node saturation 90.0%; fallbacks 0/1613.

## Tutorial

Map (0, 1); distance 0.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 93.3% [78.7, 98.2]; U=0; range 93.3–93.3% | — | — |
| Normal | 100.0% [88.6, 100.0]; U=0; range 100.0–100.0% | — | — |
| Hard | 96.7% [83.3, 99.4]; U=0; range 96.7–96.7% | — | — |

- Easy/Easy: W/L/D/U 28/2/0/0; mean 11.5 plies (7–22); 5751 nodes; node saturation 0.0%; fallbacks 0/344.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 8.9 plies (7–19); 16675 nodes; node saturation 0.0%; fallbacks 0/266.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 14.6 plies (7–34); 266581 nodes; node saturation 0.0%; fallbacks 0/437.

## Outward progression candidates

Normal/Normal comparisons across cardinal edges of increasing shortest tutorial distance. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.

- Basics I → Basics II: 33.3-point drop; supported candidate.
- Basics III → Basics IV: 46.7-point drop; supported candidate.
- Basics III → Challenge I: 50.0-point drop; supported candidate.
- Patterns II → Diagonals I: 23.3-point drop; followup candidate.
- Patterns III → Beams I: 26.7-point drop; followup candidate.
- Patterns III → Challenge II: 36.7-point drop; supported candidate.
- Diagonals III → Shields I: 46.7-point drop; supported candidate.
- Challenge II → Rite II: 20.0-point drop; followup candidate.
- Rite I → Challenge III: 33.3-point drop; followup candidate.
- Ascension I → Ascension II: 26.7-point drop; followup candidate.

## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Basics II: player Easy→Hard sensitivity against Normal: +26.7 points.
- Basics III: player Easy→Hard sensitivity against Normal: +46.7 points.
- Basics IV: player Easy→Hard sensitivity against Normal: +66.7 points.
- Patterns I: player Easy→Hard sensitivity against Normal: +73.3 points.
- Patterns I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns II: player Easy→Hard sensitivity against Normal: +60.0 points.
- Patterns III: player Easy→Hard sensitivity against Normal: +66.7 points.
- Patterns III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals I: player Easy→Hard sensitivity against Normal: +43.3 points.
- Diagonals II: player Easy→Hard sensitivity against Normal: +60.0 points.
- Diagonals II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals III: player Easy→Hard sensitivity against Normal: +63.3 points.
- Diagonals III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV: player Easy→Hard sensitivity against Normal: +53.3 points.
- Diagonals IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams I: player Easy→Hard sensitivity against Normal: +66.7 points.
- Beams II: player Easy→Hard sensitivity against Normal: +53.3 points.
- Beams II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams II Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Beams II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Beams II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Beams II Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams III: player Easy→Hard sensitivity against Normal: +76.7 points.
- Beams III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Shields I: player Easy→Hard sensitivity against Normal: +26.7 points.
- Shields II: player Easy→Hard sensitivity against Normal: +30.0 points.
- Shields II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Shields II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Shields II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Shields II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Shields II Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Shields III: player Easy→Hard sensitivity against Normal: +50.0 points.
- Challenge I: player Easy→Hard sensitivity against Normal: +63.3 points.
- Challenge II: player Easy→Hard sensitivity against Normal: +56.7 points.
- Challenge III: player Easy→Hard sensitivity against Normal: +40.0 points.
- Challenge III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV: player Easy→Hard sensitivity against Normal: +50.0 points.
- Challenge IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I: player Easy→Hard sensitivity against Normal: +70.0 points.
- Rite I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite II Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III: player Easy→Hard sensitivity against Normal: +60.0 points.
- Rite III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV: player Easy→Hard sensitivity against Normal: +60.0 points.
- Rite IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I: player Easy→Hard sensitivity against Normal: +60.0 points.
- Ascension I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II: player Easy→Hard sensitivity against Normal: +26.7 points.
- Ascension II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
