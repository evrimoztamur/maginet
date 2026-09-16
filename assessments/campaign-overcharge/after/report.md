# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 264 / 273 matchups complete, 7920 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-23982e7547852d42`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Basics I

Map (0, 0); distance 1.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

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
| Easy | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

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
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 20/8/2/0; mean 14.8 plies (5–25); 13561 nodes; node saturation 0.0%; fallbacks 0/443.

- Easy/Normal: W/L/D/U 17/12/1/0; mean 14.2 plies (5–22); 64029 nodes; node saturation 0.0%; fallbacks 0/425.

- Easy/Hard: W/L/D/U 16/13/1/0; mean 8.7 plies (5–22); 711515 nodes; node saturation 0.8%; fallbacks 0/261.

- Normal/Easy: W/L/D/U 28/1/1/0; mean 10.8 plies (5–21); 75211 nodes; node saturation 0.0%; fallbacks 0/323.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 10.0 plies (5–22); 115942 nodes; node saturation 0.0%; fallbacks 0/300.

- Normal/Hard: W/L/D/U 22/8/0/0; mean 7.9 plies (5–22); 786708 nodes; node saturation 1.7%; fallbacks 0/237.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 16.0 plies (5–22); 1145012 nodes; node saturation 7.1%; fallbacks 0/479.

- Hard/Normal: W/L/D/U 29/0/1/0; mean 18.5 plies (5–22); 1383438 nodes; node saturation 6.7%; fallbacks 0/554.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 13.2 plies (5–36); 2392986 nodes; node saturation 11.1%; fallbacks 0/397.

## Basics IV

Map (2, -1); distance 4.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 28/2/0/0; mean 16.3 plies (7–25); 11603 nodes; node saturation 0.0%; fallbacks 0/489.

- Easy/Normal: W/L/D/U 20/9/1/0; mean 16.5 plies (5–26); 50379 nodes; node saturation 0.0%; fallbacks 0/495.

- Easy/Hard: W/L/D/U 24/5/1/0; mean 16.3 plies (7–30); 685536 nodes; node saturation 0.0%; fallbacks 0/489.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 14.0 plies (7–23); 56023 nodes; node saturation 0.0%; fallbacks 0/419.

- Normal/Normal: W/L/D/U 29/1/0/0; mean 12.0 plies (7–21); 85826 nodes; node saturation 0.0%; fallbacks 0/359.

- Normal/Hard: W/L/D/U 29/1/0/0; mean 11.6 plies (7–21); 613043 nodes; node saturation 0.0%; fallbacks 0/349.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 14.6 plies (7–23); 799645 nodes; node saturation 0.0%; fallbacks 0/439.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 14.1 plies (7–26); 902902 nodes; node saturation 0.0%; fallbacks 0/423.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 14.5 plies (7–24); 1407137 nodes; node saturation 0.0%; fallbacks 0/435.

## Patterns I

Map (3, -1); distance 5.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |

- Easy/Easy: W/L/D/U 14/16/0/0; mean 27.5 plies (16–45); 32474 nodes; node saturation 0.0%; fallbacks 0/826.

- Easy/Normal: W/L/D/U 8/21/1/0; mean 26.8 plies (14–44); 230415 nodes; node saturation 0.0%; fallbacks 0/805.

- Easy/Hard: W/L/D/U 3/26/1/0; mean 26.4 plies (14–39); 3739128 nodes; node saturation 16.8%; fallbacks 0/791.

- Normal/Easy: W/L/D/U 24/5/1/0; mean 26.9 plies (19–42); 224117 nodes; node saturation 0.0%; fallbacks 0/807.

- Normal/Normal: W/L/D/U 19/8/3/0; mean 25.4 plies (17–39); 410533 nodes; node saturation 0.0%; fallbacks 0/763.

- Normal/Hard: W/L/D/U 5/24/1/0; mean 27.4 plies (17–40); 4457994 nodes; node saturation 19.6%; fallbacks 0/821.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 29.7 plies (21–47); 4623588 nodes; node saturation 18.4%; fallbacks 0/892.

- Hard/Normal: W/L/D/U 29/0/1/0; mean 29.6 plies (17–45); 4727355 nodes; node saturation 19.3%; fallbacks 0/887.

- Hard/Hard: W/L/D/U 20/7/3/0; mean 27.2 plies (14–39); 8456967 nodes; node saturation 37.8%; fallbacks 0/815.

## Patterns II

Map (3, -2); distance 6.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |

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

Map (4, -2); distance 7.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 14/11/5/0; mean 34.8 plies (27–52); 78914 nodes; node saturation 0.0%; fallbacks 0/1045.

- Easy/Normal: W/L/D/U 9/16/5/0; mean 38.0 plies (27–58); 778764 nodes; node saturation 0.0%; fallbacks 0/1141.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 35.8 plies (27–52); 7893775 nodes; node saturation 32.0%; fallbacks 0/1075.

- Normal/Easy: W/L/D/U 23/5/2/0; mean 32.9 plies (27–48); 801252 nodes; node saturation 0.0%; fallbacks 0/987.

- Normal/Normal: W/L/D/U 21/3/6/0; mean 37.0 plies (25–55); 1629330 nodes; node saturation 0.0%; fallbacks 0/1109.

- Normal/Hard: W/L/D/U 11/16/3/0; mean 34.3 plies (21–57); 8467664 nodes; node saturation 32.7%; fallbacks 0/1029.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 33.8 plies (27–47); 8352129 nodes; node saturation 36.2%; fallbacks 0/1013.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 34.1 plies (27–46); 8891445 nodes; node saturation 35.3%; fallbacks 0/1022.

- Hard/Hard: W/L/D/U 24/5/1/0; mean 33.9 plies (27–50); 15191783 nodes; node saturation 63.1%; fallbacks 0/1018.

## Diagonals I

Map (5, -2); distance 8.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 10.1 plies (5–14); 5810 nodes; node saturation 0.0%; fallbacks 0/304.

- Easy/Normal: W/L/D/U 15/15/0/0; mean 10.8 plies (5–18); 25263 nodes; node saturation 0.0%; fallbacks 0/325.

- Easy/Hard: W/L/D/U 13/17/0/0; mean 12.1 plies (5–30); 252166 nodes; node saturation 0.0%; fallbacks 0/363.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 7.9 plies (5–15); 27793 nodes; node saturation 0.0%; fallbacks 0/237.

- Normal/Normal: W/L/D/U 29/1/0/0; mean 8.3 plies (5–15); 45353 nodes; node saturation 0.0%; fallbacks 0/249.

- Normal/Hard: W/L/D/U 26/3/1/0; mean 9.1 plies (5–19); 270263 nodes; node saturation 0.0%; fallbacks 0/274.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 14.2 plies (5–24); 410100 nodes; node saturation 0.0%; fallbacks 0/427.

- Hard/Normal: W/L/D/U 26/3/1/0; mean 14.2 plies (5–25); 446088 nodes; node saturation 0.0%; fallbacks 0/425.

- Hard/Hard: W/L/D/U 26/3/1/0; mean 13.5 plies (5–25); 734889 nodes; node saturation 0.0%; fallbacks 0/404.

## Diagonals II

Map (5, -1); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% |

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

Map (5, 0); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% |

- Easy/Easy: W/L/D/U 16/12/2/0; mean 35.7 plies (27–54); 76015 nodes; node saturation 0.0%; fallbacks 0/1071.

- Easy/Normal: W/L/D/U 5/23/2/0; mean 33.2 plies (27–43); 632285 nodes; node saturation 0.0%; fallbacks 0/996.

- Easy/Hard: W/L/D/U 2/26/2/0; mean 33.1 plies (27–54); 7570322 nodes; node saturation 33.0%; fallbacks 0/994.

- Normal/Easy: W/L/D/U 23/7/0/0; mean 35.9 plies (25–64); 663865 nodes; node saturation 0.0%; fallbacks 0/1078.

- Normal/Normal: W/L/D/U 14/12/4/0; mean 33.5 plies (21–50); 1087055 nodes; node saturation 0.0%; fallbacks 0/1004.

- Normal/Hard: W/L/D/U 10/16/4/0; mean 33.4 plies (27–48); 7701380 nodes; node saturation 30.0%; fallbacks 0/1001.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 34.4 plies (27–51); 7982898 nodes; node saturation 33.6%; fallbacks 0/1032.

- Hard/Normal: W/L/D/U 24/4/2/0; mean 30.9 plies (25–50); 7789609 nodes; node saturation 35.1%; fallbacks 0/927.

- Hard/Hard: W/L/D/U 19/9/2/0; mean 33.5 plies (21–51); 14097896 nodes; node saturation 60.4%; fallbacks 0/1006.

## Diagonals IV

Map (4, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |

- Easy/Easy: W/L/D/U 15/13/2/0; mean 39.5 plies (27–52); 82805 nodes; node saturation 0.0%; fallbacks 0/1186.

- Easy/Normal: W/L/D/U 5/18/7/0; mean 38.5 plies (27–58); 711729 nodes; node saturation 0.0%; fallbacks 0/1155.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 34.9 plies (27–57); 9160304 nodes; node saturation 40.1%; fallbacks 0/1047.

- Normal/Easy: W/L/D/U 21/4/5/0; mean 36.5 plies (27–61); 764530 nodes; node saturation 0.0%; fallbacks 0/1095.

- Normal/Normal: W/L/D/U 10/11/9/0; mean 38.0 plies (27–61); 1468891 nodes; node saturation 0.0%; fallbacks 0/1139.

- Normal/Hard: W/L/D/U 3/19/8/0; mean 36.7 plies (27–64); 9716164 nodes; node saturation 36.9%; fallbacks 0/1102.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 34.7 plies (27–51); 9211990 nodes; node saturation 39.2%; fallbacks 0/1040.

- Hard/Normal: W/L/D/U 21/5/4/0; mean 35.8 plies (27–50); 9629076 nodes; node saturation 38.1%; fallbacks 0/1075.

- Hard/Hard: W/L/D/U 13/11/6/0; mean 38.7 plies (27–51); 19316463 nodes; node saturation 74.9%; fallbacks 0/1161.

## Beams I

Map (6, -2); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 14/16/0/0; mean 16.3 plies (9–28); 16146 nodes; node saturation 0.0%; fallbacks 0/488.

- Easy/Normal: W/L/D/U 9/20/1/0; mean 15.3 plies (9–24); 81907 nodes; node saturation 0.0%; fallbacks 0/458.

- Easy/Hard: W/L/D/U 3/26/1/0; mean 18.3 plies (12–30); 2015602 nodes; node saturation 2.2%; fallbacks 0/550.

- Normal/Easy: W/L/D/U 25/4/1/0; mean 14.5 plies (9–23); 82519 nodes; node saturation 0.0%; fallbacks 0/434.

- Normal/Normal: W/L/D/U 19/11/0/0; mean 14.8 plies (9–28); 147125 nodes; node saturation 0.0%; fallbacks 0/445.

- Normal/Hard: W/L/D/U 21/9/0/0; mean 13.8 plies (9–26); 1764018 nodes; node saturation 3.1%; fallbacks 0/415.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 16.6 plies (9–26); 1790935 nodes; node saturation 6.8%; fallbacks 0/499.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 15.1 plies (11–30); 1909305 nodes; node saturation 7.7%; fallbacks 0/454.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 18.2 plies (9–29); 3666983 nodes; node saturation 6.8%; fallbacks 0/545.

## Beams II

Map (6, -3); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |

- Easy/Easy: W/L/D/U 7/17/6/0; mean 38.1 plies (27–69); 57720 nodes; node saturation 0.0%; fallbacks 0/1144.

- Easy/Normal: W/L/D/U 5/22/3/0; mean 36.7 plies (27–58); 415716 nodes; node saturation 0.0%; fallbacks 0/1101.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 33.8 plies (24–52); 7491622 nodes; node saturation 29.5%; fallbacks 0/1013.

- Normal/Easy: W/L/D/U 21/5/4/0; mean 40.9 plies (21–63); 480188 nodes; node saturation 0.0%; fallbacks 0/1228.

- Normal/Normal: W/L/D/U 11/11/8/0; mean 40.2 plies (27–58); 858508 nodes; node saturation 0.0%; fallbacks 0/1206.

- Normal/Hard: W/L/D/U 5/18/7/0; mean 37.2 plies (27–54); 8430204 nodes; node saturation 28.3%; fallbacks 0/1115.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 35.8 plies (27–53); 7973341 nodes; node saturation 29.9%; fallbacks 0/1073.

- Hard/Normal: W/L/D/U 21/3/6/0; mean 37.1 plies (27–72); 8694212 nodes; node saturation 30.4%; fallbacks 0/1112.

- Hard/Hard: W/L/D/U 13/9/8/0; mean 35.3 plies (27–68); 14707242 nodes; node saturation 51.1%; fallbacks 0/1058.

## Beams III

Map (6, -4); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |

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

Map (7, -2); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 30/0/0/0; mean 6.7 plies (5–13); 2794 nodes; node saturation 0.0%; fallbacks 0/202.

- Easy/Normal: W/L/D/U 30/0/0/0; mean 6.5 plies (5–13); 8386 nodes; node saturation 0.0%; fallbacks 0/196.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 6.3 plies (5–11); 29220 nodes; node saturation 0.0%; fallbacks 0/190.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 5.3 plies (5–7); 7170 nodes; node saturation 0.0%; fallbacks 0/160.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 6.1 plies (5–17); 12266 nodes; node saturation 0.0%; fallbacks 0/182.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 5.9 plies (5–9); 29539 nodes; node saturation 0.0%; fallbacks 0/176.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 7.7 plies (5–15); 48942 nodes; node saturation 0.0%; fallbacks 0/230.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 8.9 plies (5–15); 63106 nodes; node saturation 0.0%; fallbacks 0/266.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 8.7 plies (5–17); 95230 nodes; node saturation 0.0%; fallbacks 0/260.

## Shields II

Map (7, -1); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |

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

Map (7, 0); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 21.3 plies (13–32); 24941 nodes; node saturation 0.0%; fallbacks 0/638.

- Easy/Normal: W/L/D/U 9/20/1/0; mean 20.8 plies (12–36); 163897 nodes; node saturation 0.0%; fallbacks 0/625.

- Easy/Hard: W/L/D/U 3/26/1/0; mean 20.3 plies (11–32); 3148687 nodes; node saturation 18.7%; fallbacks 0/609.

- Normal/Easy: W/L/D/U 19/4/7/0; mean 19.6 plies (13–31); 155475 nodes; node saturation 0.0%; fallbacks 0/588.

- Normal/Normal: W/L/D/U 16/12/2/0; mean 20.2 plies (11–32); 285831 nodes; node saturation 0.0%; fallbacks 0/607.

- Normal/Hard: W/L/D/U 7/20/3/0; mean 19.9 plies (15–32); 3039960 nodes; node saturation 17.1%; fallbacks 0/597.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 19.2 plies (11–31); 3486984 nodes; node saturation 23.6%; fallbacks 0/576.

- Hard/Normal: W/L/D/U 24/2/4/0; mean 20.3 plies (13–45); 3852204 nodes; node saturation 24.2%; fallbacks 0/608.

- Hard/Hard: W/L/D/U 23/6/1/0; mean 16.3 plies (11–28); 5476543 nodes; node saturation 41.1%; fallbacks 0/489.

## Challenge I

Map (8, -4); distance 13.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |

- Easy/Easy: W/L/D/U 13/17/0/0; mean 16.3 plies (12–25); 20311 nodes; node saturation 0.0%; fallbacks 0/488.

- Easy/Normal: W/L/D/U 5/24/1/0; mean 17.2 plies (12–28); 139939 nodes; node saturation 0.0%; fallbacks 0/517.

- Easy/Hard: W/L/D/U 0/27/3/0; mean 19.2 plies (11–29); 2897771 nodes; node saturation 18.5%; fallbacks 0/577.

- Normal/Easy: W/L/D/U 19/10/1/0; mean 18.0 plies (11–34); 138481 nodes; node saturation 0.0%; fallbacks 0/539.

- Normal/Normal: W/L/D/U 11/10/9/0; mean 17.9 plies (12–25); 273278 nodes; node saturation 0.0%; fallbacks 0/537.

- Normal/Hard: W/L/D/U 3/24/3/0; mean 17.8 plies (12–29); 3114944 nodes; node saturation 21.7%; fallbacks 0/535.

- Hard/Easy: W/L/D/U 27/1/2/0; mean 18.9 plies (13–25); 3052810 nodes; node saturation 20.3%; fallbacks 0/566.

- Hard/Normal: W/L/D/U 24/3/3/0; mean 19.7 plies (11–29); 3427223 nodes; node saturation 23.3%; fallbacks 0/592.

- Hard/Hard: W/L/D/U 16/8/6/0; mean 18.4 plies (12–31); 6333095 nodes; node saturation 45.8%; fallbacks 0/552.

## Challenge II

Map (8, -5); distance 14.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Hard | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |

- Easy/Easy: W/L/D/U 9/13/8/0; mean 27.4 plies (16–40); 27493 nodes; node saturation 0.0%; fallbacks 0/821.

- Easy/Normal: W/L/D/U 5/18/7/0; mean 23.7 plies (13–39); 160149 nodes; node saturation 0.0%; fallbacks 0/711.

- Easy/Hard: W/L/D/U 1/25/4/0; mean 21.1 plies (12–37); 3409841 nodes; node saturation 18.0%; fallbacks 0/634.

- Normal/Easy: W/L/D/U 19/6/5/0; mean 21.3 plies (11–29); 147285 nodes; node saturation 0.0%; fallbacks 0/639.

- Normal/Normal: W/L/D/U 10/11/9/0; mean 20.4 plies (10–39); 250190 nodes; node saturation 0.0%; fallbacks 0/612.

- Normal/Hard: W/L/D/U 4/21/5/0; mean 20.1 plies (12–28); 3392591 nodes; node saturation 19.8%; fallbacks 0/602.

- Hard/Easy: W/L/D/U 23/2/5/0; mean 22.1 plies (15–37); 3509274 nodes; node saturation 18.7%; fallbacks 0/663.

- Hard/Normal: W/L/D/U 22/4/4/0; mean 21.9 plies (13–29); 3086961 nodes; node saturation 17.8%; fallbacks 0/657.

- Hard/Hard: W/L/D/U 9/18/3/0; mean 23.5 plies (14–39); 7553767 nodes; node saturation 40.4%; fallbacks 0/706.

## Challenge III

Map (8, -6); distance 15.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |

- Easy/Easy: W/L/D/U 11/15/4/0; mean 45.5 plies (27–79); 104276 nodes; node saturation 0.0%; fallbacks 0/1366.

- Easy/Normal: W/L/D/U 0/30/0/0; mean 35.4 plies (27–51); 816203 nodes; node saturation 0.1%; fallbacks 0/1061.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 34.8 plies (27–58); 9194415 nodes; node saturation 39.9%; fallbacks 0/1045.

- Normal/Easy: W/L/D/U 23/4/3/0; mean 39.2 plies (27–59); 917917 nodes; node saturation 0.0%; fallbacks 0/1175.

- Normal/Normal: W/L/D/U 7/14/9/0; mean 40.7 plies (27–62); 1674094 nodes; node saturation 0.0%; fallbacks 0/1221.

- Normal/Hard: W/L/D/U 0/27/3/0; mean 34.9 plies (27–64); 9750333 nodes; node saturation 39.2%; fallbacks 0/1047.

- Hard/Easy: W/L/D/U 26/2/2/0; mean 39.0 plies (27–57); 10567599 nodes; node saturation 41.6%; fallbacks 0/1170.

- Hard/Normal: W/L/D/U 12/10/8/0; mean 40.4 plies (27–60); 11091912 nodes; node saturation 38.7%; fallbacks 0/1211.

- Hard/Hard: W/L/D/U 2/27/1/0; mean 37.2 plies (27–49); 18719165 nodes; node saturation 75.6%; fallbacks 0/1117.

## Challenge IV

Map (9, -6); distance 16.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |

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

Map (8, -2); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% |

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

Map (9, -2); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |

- Easy/Easy: W/L/D/U 15/14/1/0; mean 18.9 plies (12–30); 18108 nodes; node saturation 0.0%; fallbacks 0/567.

- Easy/Normal: W/L/D/U 7/23/0/0; mean 17.3 plies (12–26); 109644 nodes; node saturation 0.0%; fallbacks 0/520.

- Easy/Hard: W/L/D/U 5/25/0/0; mean 19.9 plies (12–30); 2786461 nodes; node saturation 8.2%; fallbacks 0/597.

- Normal/Easy: W/L/D/U 22/7/1/0; mean 17.8 plies (13–31); 122498 nodes; node saturation 0.0%; fallbacks 0/533.

- Normal/Normal: W/L/D/U 18/12/0/0; mean 18.7 plies (14–27); 209589 nodes; node saturation 0.0%; fallbacks 0/560.

- Normal/Hard: W/L/D/U 11/19/0/0; mean 21.6 plies (15–36); 3235827 nodes; node saturation 11.1%; fallbacks 0/647.

- Hard/Easy: W/L/D/U 26/4/0/0; mean 21.7 plies (14–35); 3545823 nodes; node saturation 12.6%; fallbacks 0/650.

- Hard/Normal: W/L/D/U 20/10/0/0; mean 21.3 plies (14–33); 3640845 nodes; node saturation 12.2%; fallbacks 0/638.

- Hard/Hard: W/L/D/U 10/20/0/0; mean 22.1 plies (14–31); 6974152 nodes; node saturation 29.4%; fallbacks 0/663.

## Rite III

Map (9, -3); distance 13.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% |

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

Map (9, -4); distance 14.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |

- Easy/Easy: W/L/D/U 20/7/3/0; mean 24.9 plies (16–37); 29842 nodes; node saturation 0.0%; fallbacks 0/746.

- Easy/Normal: W/L/D/U 4/23/3/0; mean 24.6 plies (14–46); 215539 nodes; node saturation 0.0%; fallbacks 0/739.

- Easy/Hard: W/L/D/U 4/25/1/0; mean 25.9 plies (16–37); 3790601 nodes; node saturation 17.9%; fallbacks 0/777.

- Normal/Easy: W/L/D/U 29/0/1/0; mean 21.2 plies (13–33); 221221 nodes; node saturation 0.0%; fallbacks 0/636.

- Normal/Normal: W/L/D/U 22/5/3/0; mean 27.2 plies (17–36); 413626 nodes; node saturation 0.0%; fallbacks 0/817.

- Normal/Hard: W/L/D/U 12/16/2/0; mean 25.3 plies (15–45); 3993919 nodes; node saturation 18.9%; fallbacks 0/760.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 22.9 plies (15–33); 4073697 nodes; node saturation 23.3%; fallbacks 0/686.

- Hard/Normal: W/L/D/U 27/1/2/0; mean 24.5 plies (17–35); 3992495 nodes; node saturation 20.7%; fallbacks 0/736.

- Hard/Hard: W/L/D/U 26/2/2/0; mean 26.5 plies (17–35); 8094765 nodes; node saturation 37.2%; fallbacks 0/795.

## Ascension I

Map (10, -4); distance 15.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 15/10/5/0; mean 42.2 plies (27–61); 109888 nodes; node saturation 0.0%; fallbacks 0/1266.

- Easy/Normal: W/L/D/U 5/20/5/0; mean 40.5 plies (27–59); 985841 nodes; node saturation 0.0%; fallbacks 0/1216.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 37.1 plies (27–52); 10076537 nodes; node saturation 43.0%; fallbacks 0/1112.

- Normal/Easy: W/L/D/U 20/3/7/0; mean 41.7 plies (27–57); 1081758 nodes; node saturation 0.4%; fallbacks 0/1252.

- Normal/Normal: W/L/D/U 15/9/6/0; mean 40.7 plies (27–56); 2069559 nodes; node saturation 1.8%; fallbacks 0/1221.

- Normal/Hard: W/L/D/U 5/24/1/0; mean 39.3 plies (27–56); 11188854 nodes; node saturation 41.3%; fallbacks 0/1178.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 36.9 plies (27–60); 10206692 nodes; node saturation 43.5%; fallbacks 0/1107.

- Hard/Normal: W/L/D/U 23/3/4/0; mean 40.4 plies (27–57); 11464478 nodes; node saturation 40.3%; fallbacks 0/1211.

- Hard/Hard: W/L/D/U 14/12/4/0; mean 41.1 plies (28–55); 21012002 nodes; node saturation 78.2%; fallbacks 0/1232.

## Ascension II

Map (11, -4); distance 16.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |

- Easy/Easy: W/L/D/U 6/18/6/0; mean 51.5 plies (33–73); 198542 nodes; node saturation 0.0%; fallbacks 0/1546.

- Easy/Normal: W/L/D/U 8/20/2/0; mean 51.0 plies (33–67); 2148260 nodes; node saturation 5.8%; fallbacks 0/1531.

- Easy/Hard: W/L/D/U 1/28/1/0; mean 47.8 plies (33–74); 13819007 nodes; node saturation 47.0%; fallbacks 0/1433.

- Normal/Easy: W/L/D/U 19/3/8/0; mean 48.2 plies (33–74); 1971502 nodes; node saturation 3.0%; fallbacks 0/1446.

- Normal/Normal: W/L/D/U 7/18/5/0; mean 51.8 plies (33–70); 4017260 nodes; node saturation 10.4%; fallbacks 0/1555.

- Normal/Hard: W/L/D/U 2/25/3/0; mean 50.2 plies (33–94); 15986842 nodes; node saturation 49.4%; fallbacks 0/1507.

- Hard/Easy: W/L/D/U 22/2/6/0; mean 52.5 plies (33–82); 15049373 nodes; node saturation 46.1%; fallbacks 0/1576.

- Hard/Normal: W/L/D/U 16/7/7/0; mean 52.5 plies (34–81); 17160441 nodes; node saturation 50.1%; fallbacks 0/1576.

- Hard/Hard: W/L/D/U 7/21/2/0; mean 53.8 plies (36–86); 29980785 nodes; node saturation 90.0%; fallbacks 0/1613.

## Crossfire

Map (7, -4); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 29/0/1/0; mean 2.2 plies (1–18); 963 nodes; node saturation 0.0%; fallbacks 0/67.

- Easy/Normal: W/L/D/U 29/0/1/0; mean 2.2 plies (1–18); 2136 nodes; node saturation 0.0%; fallbacks 0/67.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 2.3 plies (1–23); 12048 nodes; node saturation 0.0%; fallbacks 0/68.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 1.5 plies (1–7); 3531 nodes; node saturation 0.0%; fallbacks 0/44.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 1.5 plies (1–7); 3945 nodes; node saturation 0.0%; fallbacks 0/44.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 1.4 plies (1–5); 5140 nodes; node saturation 0.0%; fallbacks 0/42.

- Hard/Easy: W/L/D/U 28/2/0/0; mean 4.5 plies (1–19); 45532 nodes; node saturation 0.0%; fallbacks 0/134.

- Hard/Normal: W/L/D/U 27/3/0/0; mean 5.3 plies (1–19); 53490 nodes; node saturation 0.0%; fallbacks 0/159.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 3.9 plies (1–19); 52695 nodes; node saturation 0.0%; fallbacks 0/118.

## Side Step

Map (6, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 11/3/16/0; mean 11.8 plies (3–15); 6300 nodes; node saturation 0.0%; fallbacks 0/353.

- Easy/Normal: W/L/D/U 12/3/15/0; mean 11.3 plies (3–15); 21340 nodes; node saturation 0.0%; fallbacks 0/339.

- Easy/Hard: W/L/D/U 10/14/6/0; mean 10.5 plies (3–15); 114038 nodes; node saturation 0.0%; fallbacks 0/316.

- Normal/Easy: W/L/D/U 23/1/6/0; mean 10.0 plies (3–15); 23740 nodes; node saturation 0.0%; fallbacks 0/301.

- Normal/Normal: W/L/D/U 21/3/6/0; mean 9.0 plies (3–15); 33548 nodes; node saturation 0.0%; fallbacks 0/271.

- Normal/Hard: W/L/D/U 18/4/8/0; mean 10.0 plies (3–15); 123674 nodes; node saturation 0.0%; fallbacks 0/300.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 12.2 plies (3–15); 153470 nodes; node saturation 0.0%; fallbacks 0/366.

- Hard/Normal: W/L/D/U 28/0/2/0; mean 9.5 plies (3–15); 145317 nodes; node saturation 0.0%; fallbacks 0/284.

- Hard/Hard: W/L/D/U 24/2/4/0; mean 9.6 plies (3–15); 235135 nodes; node saturation 0.0%; fallbacks 0/288.

## Ascension III

Randomized Chaos teams; excluded from fixed-scenario statistics.

## Tutorial

Map (0, 1); distance 0.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | — | — |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | — | — |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | — | — |

- Easy/Easy: W/L/D/U 30/0/0/0; mean 9.3 plies (7–21); 4079 nodes; node saturation 0.0%; fallbacks 0/280.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 8.7 plies (7–15); 13685 nodes; node saturation 0.0%; fallbacks 0/260.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 10.5 plies (7–21); 84347 nodes; node saturation 0.0%; fallbacks 0/314.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.

- Basics I → Basics II: 33.3-point drop; supported candidate.
- Basics II → Basics III: -20.0-point drop; below spike threshold.
- Basics III → Basics IV: -10.0-point drop; below spike threshold.
- Basics IV → Patterns I: 33.3-point drop; supported candidate.
- Patterns I → Patterns II: 3.3-point drop; below spike threshold.
- Patterns II → Patterns III: -10.0-point drop; below spike threshold.
- Patterns III → Diagonals I: -26.7-point drop; below spike threshold.
- Diagonals I → Beams I: 33.3-point drop; supported candidate.
- Beams I → Shields I: -36.7-point drop; below spike threshold.
- Shields I → Rite I: 43.3-point drop; supported candidate.
- Rite I → Rite II: -3.3-point drop; below spike threshold.
- Rite II → Rite III: 16.7-point drop; below spike threshold.
- Rite III → Rite IV: -30.0-point drop; below spike threshold.
- Rite IV → Ascension I: 23.3-point drop; followup candidate.
- Ascension I → Ascension II: 26.7-point drop; followup candidate.

### Optional route 1 (dead end)

- Diagonals I → Diagonals II: 53.3-point drop; supported candidate.
- Diagonals II → Diagonals III: -3.3-point drop; below spike threshold.
- Diagonals III → Diagonals IV: 13.3-point drop; below spike threshold.

### Optional route 2 (dead end)

- Beams I → Beams II: 26.7-point drop; followup candidate.
- Beams II → Beams III: -3.3-point drop; below spike threshold.

### Optional route 3 (dead end)

- Beams III → Crossfire: -60.0-point drop; below spike threshold.
- Crossfire → Challenge I: 63.3-point drop; supported candidate.

### Optional route 4 (dead end)

- Diagonals III → Side Step: -23.3-point drop; below spike threshold.
- Side Step → Shields III: 16.7-point drop; below spike threshold.

### Optional route 5 (dead end)


### Optional route 6 (dead end)

- Shields I → Shields II: 83.3-point drop; supported candidate.
- Shields II → Shields III: -36.7-point drop; below spike threshold.

### Optional route 7 (dead end)

- Rite IV → Challenge I: 36.7-point drop; supported candidate.
- Challenge I → Challenge II: 3.3-point drop; below spike threshold.
- Challenge II → Challenge III: 10.0-point drop; below spike threshold.
- Challenge III → Challenge IV: -13.3-point drop; below spike threshold.

## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Basics II: player Easy→Hard sensitivity against Normal: +26.7 points.
- Basics III: player Easy→Hard sensitivity against Normal: +40.0 points.
- Basics IV: player Easy→Hard sensitivity against Normal: +33.3 points.
- Patterns I: player Easy→Hard sensitivity against Normal: +70.0 points.
- Patterns II: player Easy→Hard sensitivity against Normal: +60.0 points.
- Patterns III: player Easy→Hard sensitivity against Normal: +66.7 points.
- Patterns III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals I: player Easy→Hard sensitivity against Normal: +36.7 points.
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
- Rite II: player Easy→Hard sensitivity against Normal: +43.3 points.
- Rite III: player Easy→Hard sensitivity against Normal: +60.0 points.
- Rite III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV: player Easy→Hard sensitivity against Normal: +76.7 points.
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
- Side Step: player Easy→Hard sensitivity against Normal: +53.3 points.
