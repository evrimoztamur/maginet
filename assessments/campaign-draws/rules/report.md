# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 264 / 273 matchups complete, 7920 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-eabbbfce798b040c`; bounded table capacity 32768 entries per search.

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

- Easy/Easy: W/L/D/U 30/0/0/0; mean 4.3 plies (3–15); 2292 nodes; node saturation 0.0%; fallbacks 0/128.

- Easy/Normal: W/L/D/U 30/0/0/0; mean 3.8 plies (3–13); 4806 nodes; node saturation 0.0%; fallbacks 0/114.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 4.1 plies (3–15); 19956 nodes; node saturation 0.0%; fallbacks 0/122.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 3.3 plies (3–9); 6811 nodes; node saturation 0.0%; fallbacks 0/98.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 3.0 plies (3–3); 8595 nodes; node saturation 0.0%; fallbacks 0/90.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 3.0 plies (3–3); 18048 nodes; node saturation 0.0%; fallbacks 0/90.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 6.6 plies (3–23); 49539 nodes; node saturation 0.0%; fallbacks 0/198.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 5.3 plies (3–17); 48367 nodes; node saturation 0.0%; fallbacks 0/158.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 5.6 plies (3–19); 72237 nodes; node saturation 0.0%; fallbacks 0/168.

## Basics II

Map (1, 0); distance 2.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 17/13/0/0; mean 6.8 plies (3–26); 4696 nodes; node saturation 0.0%; fallbacks 0/205.

- Easy/Normal: W/L/D/U 15/15/0/0; mean 4.7 plies (3–9); 12993 nodes; node saturation 0.0%; fallbacks 0/141.

- Easy/Hard: W/L/D/U 15/15/0/0; mean 6.3 plies (3–23); 98386 nodes; node saturation 0.0%; fallbacks 0/189.

- Normal/Easy: W/L/D/U 22/8/0/0; mean 4.3 plies (3–10); 17407 nodes; node saturation 0.0%; fallbacks 0/128.

- Normal/Normal: W/L/D/U 20/10/0/0; mean 4.1 plies (3–11); 23418 nodes; node saturation 0.0%; fallbacks 0/124.

- Normal/Hard: W/L/D/U 20/10/0/0; mean 4.9 plies (3–19); 108218 nodes; node saturation 0.0%; fallbacks 0/148.

- Hard/Easy: W/L/D/U 25/5/0/0; mean 5.8 plies (3–25); 233404 nodes; node saturation 0.0%; fallbacks 0/175.

- Hard/Normal: W/L/D/U 23/7/0/0; mean 5.4 plies (3–26); 238070 nodes; node saturation 0.0%; fallbacks 0/162.

- Hard/Hard: W/L/D/U 23/7/0/0; mean 5.3 plies (3–23); 294798 nodes; node saturation 0.0%; fallbacks 0/159.

## Basics III

Map (2, 0); distance 3.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 20/8/2/0; mean 16.5 plies (5–29); 14733 nodes; node saturation 0.0%; fallbacks 0/496.

- Easy/Normal: W/L/D/U 16/13/1/0; mean 17.0 plies (5–54); 72324 nodes; node saturation 0.0%; fallbacks 0/511.

- Easy/Hard: W/L/D/U 16/14/0/0; mean 9.1 plies (5–26); 725527 nodes; node saturation 0.7%; fallbacks 0/272.

- Normal/Easy: W/L/D/U 28/1/1/0; mean 11.1 plies (5–29); 75889 nodes; node saturation 0.0%; fallbacks 0/333.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 10.0 plies (5–22); 116048 nodes; node saturation 0.0%; fallbacks 0/300.

- Normal/Hard: W/L/D/U 22/8/0/0; mean 8.2 plies (5–29); 798127 nodes; node saturation 1.6%; fallbacks 0/245.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 17.1 plies (5–29); 1205208 nodes; node saturation 6.6%; fallbacks 0/514.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 23.9 plies (5–49); 1591493 nodes; node saturation 5.2%; fallbacks 0/718.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 14.1 plies (5–39); 2475834 nodes; node saturation 10.4%; fallbacks 0/423.

## Basics IV

Map (2, -1); distance 4.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 27/3/0/0; mean 19.0 plies (7–41); 13268 nodes; node saturation 0.0%; fallbacks 0/570.

- Easy/Normal: W/L/D/U 21/8/1/0; mean 17.5 plies (5–34); 52261 nodes; node saturation 0.0%; fallbacks 0/525.

- Easy/Hard: W/L/D/U 24/4/2/0; mean 20.2 plies (7–46); 734852 nodes; node saturation 0.0%; fallbacks 0/606.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 14.4 plies (7–29); 57076 nodes; node saturation 0.0%; fallbacks 0/433.

- Normal/Normal: W/L/D/U 28/2/0/0; mean 13.3 plies (7–32); 89238 nodes; node saturation 0.0%; fallbacks 0/398.

- Normal/Hard: W/L/D/U 29/1/0/0; mean 12.1 plies (7–29); 625045 nodes; node saturation 0.0%; fallbacks 0/363.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 16.0 plies (7–31); 808053 nodes; node saturation 0.0%; fallbacks 0/480.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 15.3 plies (7–34); 912682 nodes; node saturation 0.0%; fallbacks 0/459.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 16.1 plies (7–33); 1438398 nodes; node saturation 0.0%; fallbacks 0/483.

## Patterns I

Map (3, -1); distance 5.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |

- Easy/Easy: W/L/D/U 13/17/0/0; mean 28.9 plies (16–58); 33017 nodes; node saturation 0.0%; fallbacks 0/867.

- Easy/Normal: W/L/D/U 11/18/1/0; mean 28.3 plies (14–53); 233052 nodes; node saturation 0.0%; fallbacks 0/849.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 29.4 plies (14–48); 3764436 nodes; node saturation 15.1%; fallbacks 0/883.

- Normal/Easy: W/L/D/U 23/7/0/0; mean 27.7 plies (19–50); 225981 nodes; node saturation 0.0%; fallbacks 0/832.

- Normal/Normal: W/L/D/U 19/8/3/0; mean 27.0 plies (17–47); 414275 nodes; node saturation 0.0%; fallbacks 0/809.

- Normal/Hard: W/L/D/U 5/25/0/0; mean 29.1 plies (17–46); 4475423 nodes; node saturation 18.4%; fallbacks 0/874.

- Hard/Easy: W/L/D/U 27/2/1/0; mean 32.0 plies (21–49); 4666767 nodes; node saturation 17.1%; fallbacks 0/959.

- Hard/Normal: W/L/D/U 29/0/1/0; mean 31.2 plies (17–53); 4764618 nodes; node saturation 18.2%; fallbacks 0/937.

- Hard/Hard: W/L/D/U 20/7/3/0; mean 30.0 plies (14–45); 8513032 nodes; node saturation 34.2%; fallbacks 0/901.

## Patterns II

Map (3, -2); distance 6.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% |

- Easy/Easy: W/L/D/U 21/7/2/0; mean 42.3 plies (31–65); 39319 nodes; node saturation 0.0%; fallbacks 0/1269.

- Easy/Normal: W/L/D/U 3/26/1/0; mean 43.4 plies (28–64); 270490 nodes; node saturation 0.0%; fallbacks 0/1301.

- Easy/Hard: W/L/D/U 1/28/1/0; mean 39.5 plies (26–60); 6053109 nodes; node saturation 5.7%; fallbacks 0/1184.

- Normal/Easy: W/L/D/U 22/5/3/0; mean 42.2 plies (25–71); 239653 nodes; node saturation 0.0%; fallbacks 0/1266.

- Normal/Normal: W/L/D/U 21/8/1/0; mean 43.4 plies (29–70); 477495 nodes; node saturation 0.0%; fallbacks 0/1302.

- Normal/Hard: W/L/D/U 2/18/10/0; mean 40.1 plies (26–73); 6209480 nodes; node saturation 3.3%; fallbacks 0/1204.

- Hard/Easy: W/L/D/U 28/0/2/0; mean 40.8 plies (25–72); 6136457 nodes; node saturation 3.4%; fallbacks 0/1223.

- Hard/Normal: W/L/D/U 25/1/4/0; mean 40.4 plies (25–81); 6419935 nodes; node saturation 2.2%; fallbacks 0/1211.

- Hard/Hard: W/L/D/U 6/6/18/0; mean 40.9 plies (29–71); 12377760 nodes; node saturation 5.1%; fallbacks 0/1227.

## Patterns III

Map (4, -2); distance 7.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Normal | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% |

- Easy/Easy: W/L/D/U 13/14/3/0; mean 44.4 plies (29–72); 88230 nodes; node saturation 0.0%; fallbacks 0/1331.

- Easy/Normal: W/L/D/U 10/17/3/0; mean 47.7 plies (34–69); 857224 nodes; node saturation 0.0%; fallbacks 0/1432.

- Easy/Hard: W/L/D/U 4/26/0/0; mean 40.1 plies (26–63); 8578416 nodes; node saturation 30.2%; fallbacks 0/1202.

- Normal/Easy: W/L/D/U 26/4/0/0; mean 42.1 plies (27–64); 882752 nodes; node saturation 0.0%; fallbacks 0/1264.

- Normal/Normal: W/L/D/U 22/4/4/0; mean 47.1 plies (25–83); 1781549 nodes; node saturation 0.0%; fallbacks 0/1412.

- Normal/Hard: W/L/D/U 10/19/1/0; mean 43.9 plies (21–68); 9889900 nodes; node saturation 28.3%; fallbacks 0/1317.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 43.8 plies (27–84); 9927812 nodes; node saturation 30.7%; fallbacks 0/1314.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 45.4 plies (27–71); 10561922 nodes; node saturation 29.2%; fallbacks 0/1363.

- Hard/Hard: W/L/D/U 19/8/3/0; mean 44.1 plies (31–67); 17827476 nodes; node saturation 54.0%; fallbacks 0/1323.

## Diagonals I

Map (5, -2); distance 8.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 10.1 plies (5–14); 5810 nodes; node saturation 0.0%; fallbacks 0/304.

- Easy/Normal: W/L/D/U 15/15/0/0; mean 10.8 plies (5–17); 25336 nodes; node saturation 0.0%; fallbacks 0/323.

- Easy/Hard: W/L/D/U 13/17/0/0; mean 13.0 plies (5–36); 266506 nodes; node saturation 0.0%; fallbacks 0/391.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 7.8 plies (5–15); 27792 nodes; node saturation 0.0%; fallbacks 0/235.

- Normal/Normal: W/L/D/U 29/1/0/0; mean 8.4 plies (5–17); 45908 nodes; node saturation 0.0%; fallbacks 0/251.

- Normal/Hard: W/L/D/U 27/3/0/0; mean 9.6 plies (5–26); 282812 nodes; node saturation 0.0%; fallbacks 0/287.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 15.4 plies (5–29); 442472 nodes; node saturation 0.0%; fallbacks 0/462.

- Hard/Normal: W/L/D/U 28/2/0/0; mean 16.4 plies (5–39); 507200 nodes; node saturation 0.0%; fallbacks 0/492.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 14.9 plies (5–37); 811938 nodes; node saturation 0.0%; fallbacks 0/446.

## Diagonals II

Map (5, -1); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 14/15/1/0; mean 42.1 plies (24–78); 55342 nodes; node saturation 0.0%; fallbacks 0/1262.

- Easy/Normal: W/L/D/U 7/21/2/0; mean 39.2 plies (22–62); 376038 nodes; node saturation 0.0%; fallbacks 0/1177.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 38.1 plies (18–61); 6891395 nodes; node saturation 20.7%; fallbacks 0/1143.

- Normal/Easy: W/L/D/U 27/3/0/0; mean 37.7 plies (21–53); 395858 nodes; node saturation 0.0%; fallbacks 0/1130.

- Normal/Normal: W/L/D/U 16/12/2/0; mean 43.2 plies (24–78); 742979 nodes; node saturation 0.0%; fallbacks 0/1295.

- Normal/Hard: W/L/D/U 3/26/1/0; mean 36.6 plies (24–55); 7128874 nodes; node saturation 20.9%; fallbacks 0/1097.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 40.1 plies (23–65); 7292653 nodes; node saturation 22.1%; fallbacks 0/1203.

- Hard/Normal: W/L/D/U 21/7/2/0; mean 37.8 plies (21–67); 6914275 nodes; node saturation 20.1%; fallbacks 0/1135.

- Hard/Hard: W/L/D/U 23/6/1/0; mean 37.3 plies (21–62); 13282658 nodes; node saturation 40.5%; fallbacks 0/1120.

## Diagonals III

Map (5, 0); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% |

- Easy/Easy: W/L/D/U 14/13/3/0; mean 44.4 plies (29–68); 85921 nodes; node saturation 0.0%; fallbacks 0/1332.

- Easy/Normal: W/L/D/U 4/23/3/0; mean 44.0 plies (28–66); 701783 nodes; node saturation 0.0%; fallbacks 0/1320.

- Easy/Hard: W/L/D/U 1/27/2/0; mean 44.3 plies (32–68); 9013267 nodes; node saturation 26.4%; fallbacks 0/1328.

- Normal/Easy: W/L/D/U 22/8/0/0; mean 44.2 plies (25–74); 712823 nodes; node saturation 0.0%; fallbacks 0/1325.

- Normal/Normal: W/L/D/U 15/13/2/0; mean 46.7 plies (21–76); 1223085 nodes; node saturation 0.0%; fallbacks 0/1400.

- Normal/Hard: W/L/D/U 9/17/4/0; mean 45.0 plies (28–87); 8747166 nodes; node saturation 22.6%; fallbacks 0/1351.

- Hard/Easy: W/L/D/U 27/2/1/0; mean 43.1 plies (27–60); 9001773 nodes; node saturation 28.7%; fallbacks 0/1294.

- Hard/Normal: W/L/D/U 26/3/1/0; mean 41.9 plies (25–68); 8818563 nodes; node saturation 26.4%; fallbacks 0/1257.

- Hard/Hard: W/L/D/U 19/10/1/0; mean 42.7 plies (21–62); 15129237 nodes; node saturation 47.6%; fallbacks 0/1281.

## Diagonals IV

Map (4, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |

- Easy/Easy: W/L/D/U 11/16/3/0; mean 56.8 plies (35–88); 102672 nodes; node saturation 0.0%; fallbacks 0/1704.

- Easy/Normal: W/L/D/U 2/25/3/0; mean 56.7 plies (28–88); 889307 nodes; node saturation 0.0%; fallbacks 0/1700.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 53.6 plies (35–82); 12849729 nodes; node saturation 31.6%; fallbacks 0/1609.

- Normal/Easy: W/L/D/U 22/6/2/0; mean 54.4 plies (27–87); 977782 nodes; node saturation 0.0%; fallbacks 0/1632.

- Normal/Normal: W/L/D/U 14/9/7/0; mean 52.0 plies (35–87); 1847861 nodes; node saturation 0.0%; fallbacks 0/1559.

- Normal/Hard: W/L/D/U 1/21/8/0; mean 55.6 plies (35–107); 13104487 nodes; node saturation 30.9%; fallbacks 0/1669.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 50.7 plies (35–79); 12897895 nodes; node saturation 35.9%; fallbacks 0/1522.

- Hard/Normal: W/L/D/U 24/2/4/0; mean 50.8 plies (29–85); 13207026 nodes; node saturation 34.8%; fallbacks 0/1523.

- Hard/Hard: W/L/D/U 12/14/4/0; mean 54.7 plies (35–74); 24283291 nodes; node saturation 63.3%; fallbacks 0/1641.

## Beams I

Map (6, -2); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 14/16/0/0; mean 16.9 plies (9–29); 16403 nodes; node saturation 0.0%; fallbacks 0/508.

- Easy/Normal: W/L/D/U 9/21/0/0; mean 15.8 plies (9–29); 82719 nodes; node saturation 0.0%; fallbacks 0/473.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 18.4 plies (12–34); 2020628 nodes; node saturation 2.2%; fallbacks 0/553.

- Normal/Easy: W/L/D/U 23/7/0/0; mean 15.8 plies (9–38); 85040 nodes; node saturation 0.0%; fallbacks 0/475.

- Normal/Normal: W/L/D/U 19/11/0/0; mean 14.8 plies (9–28); 147604 nodes; node saturation 0.0%; fallbacks 0/445.

- Normal/Hard: W/L/D/U 21/9/0/0; mean 14.6 plies (9–38); 1779816 nodes; node saturation 3.0%; fallbacks 0/437.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 17.7 plies (9–33); 1802078 nodes; node saturation 6.4%; fallbacks 0/532.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 16.0 plies (11–37); 1932795 nodes; node saturation 7.3%; fallbacks 0/479.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 19.1 plies (9–41); 3744270 nodes; node saturation 6.5%; fallbacks 0/573.

## Beams II

Map (6, -3); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 10/16/4/0; mean 59.0 plies (35–115); 77101 nodes; node saturation 0.0%; fallbacks 0/1769.

- Easy/Normal: W/L/D/U 6/23/1/0; mean 56.3 plies (30–104); 526761 nodes; node saturation 0.0%; fallbacks 0/1689.

- Easy/Hard: W/L/D/U 1/29/0/0; mean 48.4 plies (24–97); 9178094 nodes; node saturation 23.5%; fallbacks 0/1453.

- Normal/Easy: W/L/D/U 21/5/4/0; mean 57.1 plies (21–122); 563561 nodes; node saturation 0.0%; fallbacks 0/1713.

- Normal/Normal: W/L/D/U 12/10/8/0; mean 57.0 plies (36–99); 1027595 nodes; node saturation 0.0%; fallbacks 0/1710.

- Normal/Hard: W/L/D/U 3/23/4/0; mean 49.9 plies (35–84); 10223439 nodes; node saturation 23.6%; fallbacks 0/1498.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 45.2 plies (29–75); 8941726 nodes; node saturation 24.8%; fallbacks 0/1356.

- Hard/Normal: W/L/D/U 20/2/8/0; mean 49.6 plies (35–87); 10526119 nodes; node saturation 25.9%; fallbacks 0/1489.

- Hard/Hard: W/L/D/U 14/8/8/0; mean 51.7 plies (35–92); 17994514 nodes; node saturation 37.2%; fallbacks 0/1550.

## Beams III

Map (6, -4); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |

- Easy/Easy: W/L/D/U 13/15/2/0; mean 37.0 plies (20–76); 69726 nodes; node saturation 0.0%; fallbacks 0/1109.

- Easy/Normal: W/L/D/U 4/26/0/0; mean 31.5 plies (18–56); 443595 nodes; node saturation 0.0%; fallbacks 0/945.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 36.9 plies (16–66); 8742909 nodes; node saturation 31.9%; fallbacks 0/1106.

- Normal/Easy: W/L/D/U 21/7/2/0; mean 40.0 plies (18–61); 701879 nodes; node saturation 0.0%; fallbacks 0/1201.

- Normal/Normal: W/L/D/U 11/16/3/0; mean 36.9 plies (17–79); 899339 nodes; node saturation 0.0%; fallbacks 0/1106.

- Normal/Hard: W/L/D/U 2/26/2/0; mean 36.1 plies (22–63); 8732360 nodes; node saturation 29.6%; fallbacks 0/1083.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 35.2 plies (19–66); 8817500 nodes; node saturation 33.0%; fallbacks 0/1057.

- Hard/Normal: W/L/D/U 25/5/0/0; mean 34.0 plies (17–55); 8550277 nodes; node saturation 32.1%; fallbacks 0/1021.

- Hard/Hard: W/L/D/U 12/18/0/0; mean 38.4 plies (19–61); 17833771 nodes; node saturation 61.3%; fallbacks 0/1151.

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

- Hard/Easy: W/L/D/U 30/0/0/0; mean 7.9 plies (5–23); 50370 nodes; node saturation 0.0%; fallbacks 0/238.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 9.4 plies (5–25); 68314 nodes; node saturation 0.0%; fallbacks 0/282.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 9.0 plies (5–25); 100459 nodes; node saturation 0.0%; fallbacks 0/270.

## Shields II

Map (7, -1); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |

- Easy/Easy: W/L/D/U 5/23/2/0; mean 27.4 plies (18–71); 37360 nodes; node saturation 0.0%; fallbacks 0/823.

- Easy/Normal: W/L/D/U 4/26/0/0; mean 25.8 plies (16–56); 257209 nodes; node saturation 0.0%; fallbacks 0/775.

- Easy/Hard: W/L/D/U 1/29/0/0; mean 27.0 plies (18–40); 5644421 nodes; node saturation 27.2%; fallbacks 0/809.

- Normal/Easy: W/L/D/U 9/19/2/0; mean 32.9 plies (18–61); 271616 nodes; node saturation 0.0%; fallbacks 0/986.

- Normal/Normal: W/L/D/U 5/23/2/0; mean 28.8 plies (16–61); 504984 nodes; node saturation 0.0%; fallbacks 0/865.

- Normal/Hard: W/L/D/U 1/28/1/0; mean 33.2 plies (18–57); 5900192 nodes; node saturation 19.5%; fallbacks 0/995.

- Hard/Easy: W/L/D/U 19/9/2/0; mean 32.7 plies (16–57); 6184745 nodes; node saturation 24.7%; fallbacks 0/981.

- Hard/Normal: W/L/D/U 15/13/2/0; mean 33.4 plies (16–54); 6248955 nodes; node saturation 23.6%; fallbacks 0/1001.

- Hard/Hard: W/L/D/U 3/25/2/0; mean 33.3 plies (16–56); 11839717 nodes; node saturation 46.7%; fallbacks 0/998.

## Shields III

Map (7, 0); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 21.8 plies (13–36); 25148 nodes; node saturation 0.0%; fallbacks 0/653.

- Easy/Normal: W/L/D/U 8/21/1/0; mean 21.7 plies (12–55); 165742 nodes; node saturation 0.0%; fallbacks 0/651.

- Easy/Hard: W/L/D/U 4/26/0/0; mean 21.2 plies (11–42); 3168337 nodes; node saturation 17.9%; fallbacks 0/636.

- Normal/Easy: W/L/D/U 20/4/6/0; mean 21.4 plies (13–33); 159384 nodes; node saturation 0.0%; fallbacks 0/641.

- Normal/Normal: W/L/D/U 17/11/2/0; mean 20.7 plies (11–33); 287792 nodes; node saturation 0.0%; fallbacks 0/622.

- Normal/Hard: W/L/D/U 9/19/2/0; mean 21.3 plies (15–36); 3062232 nodes; node saturation 15.9%; fallbacks 0/640.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 20.1 plies (11–35); 3509415 nodes; node saturation 22.6%; fallbacks 0/602.

- Hard/Normal: W/L/D/U 23/3/4/0; mean 20.8 plies (13–33); 3867070 nodes; node saturation 23.4%; fallbacks 0/624.

- Hard/Hard: W/L/D/U 23/6/1/0; mean 18.1 plies (11–39); 5603048 nodes; node saturation 37.0%; fallbacks 0/543.

## Challenge I

Map (8, -4); distance 13.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |

- Easy/Easy: W/L/D/U 13/17/0/0; mean 16.5 plies (12–33); 20431 nodes; node saturation 0.0%; fallbacks 0/496.

- Easy/Normal: W/L/D/U 5/24/1/0; mean 17.2 plies (12–28); 140159 nodes; node saturation 0.0%; fallbacks 0/517.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 21.3 plies (11–38); 2977155 nodes; node saturation 16.7%; fallbacks 0/639.

- Normal/Easy: W/L/D/U 20/9/1/0; mean 18.9 plies (11–34); 140596 nodes; node saturation 0.0%; fallbacks 0/568.

- Normal/Normal: W/L/D/U 11/14/5/0; mean 18.2 plies (12–28); 274854 nodes; node saturation 0.0%; fallbacks 0/546.

- Normal/Hard: W/L/D/U 3/25/2/0; mean 18.8 plies (12–37); 3154692 nodes; node saturation 20.5%; fallbacks 0/565.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 20.3 plies (13–37); 3096973 nodes; node saturation 18.9%; fallbacks 0/608.

- Hard/Normal: W/L/D/U 24/3/3/0; mean 22.1 plies (11–39); 3489086 nodes; node saturation 20.8%; fallbacks 0/663.

- Hard/Hard: W/L/D/U 16/8/6/0; mean 18.7 plies (12–34); 6355852 nodes; node saturation 45.0%; fallbacks 0/562.

## Challenge II

Map (8, -5); distance 14.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 12/17/1/0; mean 32.0 plies (16–61); 30139 nodes; node saturation 0.0%; fallbacks 0/960.

- Easy/Normal: W/L/D/U 7/20/3/0; mean 28.2 plies (13–47); 170258 nodes; node saturation 0.0%; fallbacks 0/845.

- Easy/Hard: W/L/D/U 1/27/2/0; mean 23.7 plies (12–46); 3474016 nodes; node saturation 16.0%; fallbacks 0/712.

- Normal/Easy: W/L/D/U 22/5/3/0; mean 25.3 plies (11–49); 156770 nodes; node saturation 0.0%; fallbacks 0/760.

- Normal/Normal: W/L/D/U 13/12/5/0; mean 28.5 plies (10–61); 282584 nodes; node saturation 0.0%; fallbacks 0/855.

- Normal/Hard: W/L/D/U 5/23/2/0; mean 24.9 plies (12–45); 3508031 nodes; node saturation 16.0%; fallbacks 0/746.

- Hard/Easy: W/L/D/U 25/1/4/0; mean 25.5 plies (15–45); 3570124 nodes; node saturation 16.2%; fallbacks 0/765.

- Hard/Normal: W/L/D/U 23/4/3/0; mean 28.3 plies (13–47); 3263419 nodes; node saturation 13.8%; fallbacks 0/849.

- Hard/Hard: W/L/D/U 14/15/1/0; mean 34.5 plies (14–62); 7949767 nodes; node saturation 27.5%; fallbacks 0/1036.

## Challenge III

Map (8, -6); distance 15.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |

- Easy/Easy: W/L/D/U 11/17/2/0; mean 57.6 plies (39–79); 121673 nodes; node saturation 0.0%; fallbacks 0/1728.

- Easy/Normal: W/L/D/U 0/29/1/0; mean 50.1 plies (28–84); 1022769 nodes; node saturation 0.1%; fallbacks 0/1503.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 49.4 plies (35–68); 11900067 nodes; node saturation 33.8%; fallbacks 0/1482.

- Normal/Easy: W/L/D/U 24/4/2/0; mean 53.9 plies (33–82); 1039878 nodes; node saturation 0.0%; fallbacks 0/1616.

- Normal/Normal: W/L/D/U 7/20/3/0; mean 56.4 plies (34–96); 1912851 nodes; node saturation 0.0%; fallbacks 0/1693.

- Normal/Hard: W/L/D/U 0/30/0/0; mean 52.6 plies (35–76); 13405752 nodes; node saturation 32.5%; fallbacks 0/1579.

- Hard/Easy: W/L/D/U 25/2/3/0; mean 53.2 plies (37–77); 12733609 nodes; node saturation 33.2%; fallbacks 0/1595.

- Hard/Normal: W/L/D/U 17/12/1/0; mean 54.6 plies (36–82); 13666981 nodes; node saturation 33.0%; fallbacks 0/1638.

- Hard/Hard: W/L/D/U 7/22/1/0; mean 52.4 plies (34–88); 22756373 nodes; node saturation 59.8%; fallbacks 0/1571.

## Challenge IV

Map (9, -6); distance 16.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |

- Easy/Easy: W/L/D/U 10/14/6/0; mean 63.5 plies (42–124); 112368 nodes; node saturation 0.0%; fallbacks 0/1906.

- Easy/Normal: W/L/D/U 1/29/0/0; mean 64.7 plies (42–94); 1059861 nodes; node saturation 0.0%; fallbacks 0/1941.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 56.3 plies (42–89); 14601663 nodes; node saturation 35.9%; fallbacks 0/1690.

- Normal/Easy: W/L/D/U 21/6/3/0; mean 68.5 plies (43–105); 995194 nodes; node saturation 0.0%; fallbacks 0/2054.

- Normal/Normal: W/L/D/U 12/12/6/0; mean 65.1 plies (50–87); 2183490 nodes; node saturation 0.0%; fallbacks 0/1954.

- Normal/Hard: W/L/D/U 2/25/3/0; mean 62.5 plies (44–92); 15808327 nodes; node saturation 32.8%; fallbacks 0/1876.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 58.5 plies (37–87); 14505706 nodes; node saturation 34.2%; fallbacks 0/1754.

- Hard/Normal: W/L/D/U 15/5/10/0; mean 64.9 plies (39–87); 16160475 nodes; node saturation 32.3%; fallbacks 0/1946.

- Hard/Hard: W/L/D/U 12/15/3/0; mean 66.9 plies (36–106); 31129395 nodes; node saturation 63.6%; fallbacks 0/2008.

## Rite I

Map (8, -2); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% |

- Easy/Easy: W/L/D/U 14/11/5/0; mean 63.3 plies (36–91); 150240 nodes; node saturation 0.0%; fallbacks 0/1900.

- Easy/Normal: W/L/D/U 2/27/1/0; mean 58.3 plies (36–103); 1533593 nodes; node saturation 1.2%; fallbacks 0/1748.

- Easy/Hard: W/L/D/U 1/29/0/0; mean 57.1 plies (34–78); 13920955 nodes; node saturation 34.7%; fallbacks 0/1714.

- Normal/Easy: W/L/D/U 29/0/1/0; mean 54.2 plies (29–111); 1593304 nodes; node saturation 1.7%; fallbacks 0/1626.

- Normal/Normal: W/L/D/U 15/12/3/0; mean 63.4 plies (40–97); 3060227 nodes; node saturation 3.1%; fallbacks 0/1902.

- Normal/Hard: W/L/D/U 10/18/2/0; mean 57.1 plies (36–118); 15580580 nodes; node saturation 38.5%; fallbacks 0/1712.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 54.0 plies (35–88); 12838450 nodes; node saturation 33.2%; fallbacks 0/1619.

- Hard/Normal: W/L/D/U 26/4/0/0; mean 54.1 plies (41–78); 14212735 nodes; node saturation 38.1%; fallbacks 0/1624.

- Hard/Hard: W/L/D/U 21/9/0/0; mean 56.9 plies (32–93); 26640656 nodes; node saturation 67.6%; fallbacks 0/1708.

## Rite II

Map (9, -2); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |

- Easy/Easy: W/L/D/U 16/12/2/0; mean 20.0 plies (12–47); 18605 nodes; node saturation 0.0%; fallbacks 0/600.

- Easy/Normal: W/L/D/U 8/22/0/0; mean 17.3 plies (12–31); 109211 nodes; node saturation 0.0%; fallbacks 0/520.

- Easy/Hard: W/L/D/U 5/25/0/0; mean 20.1 plies (12–30); 2797544 nodes; node saturation 8.1%; fallbacks 0/603.

- Normal/Easy: W/L/D/U 22/8/0/0; mean 17.8 plies (13–32); 122616 nodes; node saturation 0.0%; fallbacks 0/534.

- Normal/Normal: W/L/D/U 18/12/0/0; mean 18.9 plies (14–27); 210502 nodes; node saturation 0.0%; fallbacks 0/566.

- Normal/Hard: W/L/D/U 13/17/0/0; mean 21.9 plies (15–35); 3255754 nodes; node saturation 11.0%; fallbacks 0/657.

- Hard/Easy: W/L/D/U 26/4/0/0; mean 22.1 plies (14–43); 3553767 nodes; node saturation 12.2%; fallbacks 0/663.

- Hard/Normal: W/L/D/U 20/10/0/0; mean 22.5 plies (14–41); 3653547 nodes; node saturation 11.5%; fallbacks 0/676.

- Hard/Hard: W/L/D/U 10/20/0/0; mean 22.5 plies (14–37); 6953123 nodes; node saturation 28.9%; fallbacks 0/674.

## Rite III

Map (9, -3); distance 13.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% |

- Easy/Easy: W/L/D/U 13/15/2/0; mean 51.8 plies (29–81); 91611 nodes; node saturation 0.0%; fallbacks 0/1555.

- Easy/Normal: W/L/D/U 5/23/2/0; mean 45.3 plies (28–66); 814170 nodes; node saturation 0.0%; fallbacks 0/1358.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 43.7 plies (26–78); 9793551 nodes; node saturation 31.1%; fallbacks 0/1312.

- Normal/Easy: W/L/D/U 24/6/0/0; mean 44.9 plies (25–82); 765517 nodes; node saturation 0.0%; fallbacks 0/1346.

- Normal/Normal: W/L/D/U 18/6/6/0; mean 42.8 plies (23–69); 1505965 nodes; node saturation 0.2%; fallbacks 0/1284.

- Normal/Hard: W/L/D/U 5/22/3/0; mean 40.7 plies (25–68); 9418107 nodes; node saturation 28.8%; fallbacks 0/1221.

- Hard/Easy: W/L/D/U 27/0/3/0; mean 43.4 plies (23–69); 9986413 nodes; node saturation 32.6%; fallbacks 0/1301.

- Hard/Normal: W/L/D/U 23/5/2/0; mean 43.3 plies (25–83); 10458563 nodes; node saturation 31.4%; fallbacks 0/1298.

- Hard/Hard: W/L/D/U 8/17/5/0; mean 51.4 plies (29–75); 21481378 nodes; node saturation 57.1%; fallbacks 0/1543.

## Rite IV

Map (9, -4); distance 14.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 22/7/1/0; mean 27.1 plies (16–53); 30457 nodes; node saturation 0.0%; fallbacks 0/814.

- Easy/Normal: W/L/D/U 6/23/1/0; mean 26.8 plies (14–53); 218749 nodes; node saturation 0.0%; fallbacks 0/805.

- Easy/Hard: W/L/D/U 4/25/1/0; mean 27.7 plies (16–43); 3830178 nodes; node saturation 16.7%; fallbacks 0/831.

- Normal/Easy: W/L/D/U 29/0/1/0; mean 21.5 plies (13–41); 221666 nodes; node saturation 0.0%; fallbacks 0/644.

- Normal/Normal: W/L/D/U 22/5/3/0; mean 29.6 plies (17–43); 417441 nodes; node saturation 0.0%; fallbacks 0/888.

- Normal/Hard: W/L/D/U 12/17/1/0; mean 27.9 plies (15–53); 4019192 nodes; node saturation 17.2%; fallbacks 0/837.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 24.2 plies (15–45); 4108219 nodes; node saturation 22.0%; fallbacks 0/726.

- Hard/Normal: W/L/D/U 27/1/2/0; mean 25.6 plies (17–43); 4010544 nodes; node saturation 19.8%; fallbacks 0/767.

- Hard/Hard: W/L/D/U 24/4/2/0; mean 30.7 plies (17–64); 8172224 nodes; node saturation 32.2%; fallbacks 0/920.

## Ascension I

Map (10, -4); distance 15.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% |

- Easy/Easy: W/L/D/U 15/12/3/0; mean 61.3 plies (35–86); 135902 nodes; node saturation 0.0%; fallbacks 0/1839.

- Easy/Normal: W/L/D/U 6/21/3/0; mean 58.1 plies (36–95); 1210885 nodes; node saturation 0.0%; fallbacks 0/1743.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 52.5 plies (36–78); 13922044 nodes; node saturation 39.8%; fallbacks 0/1576.

- Normal/Easy: W/L/D/U 23/2/5/0; mean 56.7 plies (35–102); 1307846 nodes; node saturation 0.3%; fallbacks 0/1701.

- Normal/Normal: W/L/D/U 17/9/4/0; mean 59.9 plies (37–99); 2666069 nodes; node saturation 1.3%; fallbacks 0/1798.

- Normal/Hard: W/L/D/U 5/21/4/0; mean 57.6 plies (36–91); 15142271 nodes; node saturation 35.3%; fallbacks 0/1729.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 54.3 plies (37–78); 14407484 nodes; node saturation 39.5%; fallbacks 0/1628.

- Hard/Normal: W/L/D/U 27/1/2/0; mean 57.5 plies (41–93); 15139051 nodes; node saturation 34.8%; fallbacks 0/1725.

- Hard/Hard: W/L/D/U 15/11/4/0; mean 59.8 plies (43–83); 28537857 nodes; node saturation 68.0%; fallbacks 0/1795.

## Ascension II

Map (11, -4); distance 16.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% |

- Easy/Easy: W/L/D/U 9/18/3/0; mean 79.0 plies (41–108); 265768 nodes; node saturation 0.0%; fallbacks 0/2369.

- Easy/Normal: W/L/D/U 5/23/2/0; mean 85.5 plies (42–135); 3098432 nodes; node saturation 4.1%; fallbacks 0/2565.

- Easy/Hard: W/L/D/U 1/28/1/0; mean 72.7 plies (49–111); 20398700 nodes; node saturation 44.3%; fallbacks 0/2182.

- Normal/Easy: W/L/D/U 24/2/4/0; mean 76.5 plies (43–131); 2845595 nodes; node saturation 2.9%; fallbacks 0/2296.

- Normal/Normal: W/L/D/U 9/17/4/0; mean 80.0 plies (42–120); 5376996 nodes; node saturation 7.5%; fallbacks 0/2400.

- Normal/Hard: W/L/D/U 3/27/0/0; mean 80.6 plies (56–134); 24432420 nodes; node saturation 45.1%; fallbacks 0/2419.

- Hard/Easy: W/L/D/U 25/2/3/0; mean 72.3 plies (41–142); 19972526 nodes; node saturation 43.1%; fallbacks 0/2169.

- Hard/Normal: W/L/D/U 16/7/7/0; mean 82.6 plies (61–111); 24530381 nodes; node saturation 42.9%; fallbacks 0/2478.

- Hard/Hard: W/L/D/U 8/21/1/0; mean 81.4 plies (54–125); 44179513 nodes; node saturation 83.6%; fallbacks 0/2441.

## Crossfire

Map (7, -4); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 29/0/1/0; mean 2.5 plies (1–26); 1083 nodes; node saturation 0.0%; fallbacks 0/75.

- Easy/Normal: W/L/D/U 29/0/1/0; mean 2.5 plies (1–26); 2525 nodes; node saturation 0.0%; fallbacks 0/75.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 2.3 plies (1–23); 13525 nodes; node saturation 0.0%; fallbacks 0/68.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 1.5 plies (1–7); 3531 nodes; node saturation 0.0%; fallbacks 0/44.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 1.5 plies (1–7); 3945 nodes; node saturation 0.0%; fallbacks 0/44.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 1.4 plies (1–5); 5140 nodes; node saturation 0.0%; fallbacks 0/42.

- Hard/Easy: W/L/D/U 28/2/0/0; mean 4.5 plies (1–19); 45805 nodes; node saturation 0.0%; fallbacks 0/134.

- Hard/Normal: W/L/D/U 27/3/0/0; mean 5.4 plies (1–21); 54683 nodes; node saturation 0.0%; fallbacks 0/161.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 3.9 plies (1–19); 53440 nodes; node saturation 0.0%; fallbacks 0/118.

## Side Step

Map (6, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |

- Easy/Easy: W/L/D/U 11/7/12/0; mean 16.4 plies (3–28); 9092 nodes; node saturation 0.0%; fallbacks 0/491.

- Easy/Normal: W/L/D/U 13/6/11/0; mean 15.3 plies (3–26); 30869 nodes; node saturation 0.0%; fallbacks 0/458.

- Easy/Hard: W/L/D/U 10/17/3/0; mean 12.5 plies (3–30); 156883 nodes; node saturation 0.0%; fallbacks 0/375.

- Normal/Easy: W/L/D/U 24/1/5/0; mean 11.8 plies (3–23); 29359 nodes; node saturation 0.0%; fallbacks 0/355.

- Normal/Normal: W/L/D/U 22/3/5/0; mean 10.6 plies (3–23); 39769 nodes; node saturation 0.0%; fallbacks 0/319.

- Normal/Hard: W/L/D/U 19/7/4/0; mean 12.2 plies (3–23); 169531 nodes; node saturation 0.0%; fallbacks 0/367.

- Hard/Easy: W/L/D/U 28/0/2/0; mean 16.0 plies (3–23); 219619 nodes; node saturation 0.0%; fallbacks 0/480.

- Hard/Normal: W/L/D/U 28/0/2/0; mean 10.7 plies (3–23); 182606 nodes; node saturation 0.0%; fallbacks 0/322.

- Hard/Hard: W/L/D/U 26/2/2/0; mean 11.1 plies (3–23); 294586 nodes; node saturation 0.0%; fallbacks 0/334.

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
- Basics III → Basics IV: -6.7-point drop; below spike threshold.
- Basics IV → Patterns I: 30.0-point drop; supported candidate.
- Patterns I → Patterns II: -6.7-point drop; below spike threshold.
- Patterns II → Patterns III: -3.3-point drop; below spike threshold.
- Patterns III → Diagonals I: -23.3-point drop; below spike threshold.
- Diagonals I → Beams I: 33.3-point drop; supported candidate.
- Beams I → Shields I: -36.7-point drop; below spike threshold.
- Shields I → Rite I: 50.0-point drop; supported candidate.
- Rite I → Rite II: -10.0-point drop; below spike threshold.
- Rite II → Rite III: 0.0-point drop; below spike threshold.
- Rite III → Rite IV: -13.3-point drop; below spike threshold.
- Rite IV → Ascension I: 16.7-point drop; below spike threshold.
- Ascension I → Ascension II: 26.7-point drop; followup candidate.

### Optional route 1 (dead end)

- Diagonals I → Diagonals II: 43.3-point drop; supported candidate.
- Diagonals II → Diagonals III: 3.3-point drop; below spike threshold.
- Diagonals III → Diagonals IV: 3.3-point drop; below spike threshold.

### Optional route 2 (dead end)

- Beams I → Beams II: 23.3-point drop; followup candidate.
- Beams II → Beams III: 3.3-point drop; below spike threshold.

### Optional route 3 (dead end)

- Beams III → Crossfire: -63.3-point drop; below spike threshold.
- Crossfire → Challenge I: 63.3-point drop; supported candidate.

### Optional route 4 (dead end)

- Diagonals III → Side Step: -23.3-point drop; below spike threshold.
- Side Step → Shields III: 16.7-point drop; below spike threshold.

### Optional route 5 (dead end)


### Optional route 6 (dead end)

- Shields I → Shields II: 83.3-point drop; supported candidate.
- Shields II → Shields III: -40.0-point drop; below spike threshold.

### Optional route 7 (dead end)

- Rite IV → Challenge I: 36.7-point drop; supported candidate.
- Challenge I → Challenge II: -6.7-point drop; below spike threshold.
- Challenge II → Challenge III: 20.0-point drop; followup candidate.
- Challenge III → Challenge IV: -16.7-point drop; below spike threshold.

## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Basics II: player Easy→Hard sensitivity against Normal: +26.7 points.
- Basics III: player Easy→Hard sensitivity against Normal: +46.7 points.
- Basics IV: player Easy→Hard sensitivity against Normal: +30.0 points.
- Patterns I: player Easy→Hard sensitivity against Normal: +60.0 points.
- Patterns II: player Easy→Hard sensitivity against Normal: +73.3 points.
- Patterns III: player Easy→Hard sensitivity against Normal: +63.3 points.
- Patterns III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals I: player Easy→Hard sensitivity against Normal: +43.3 points.
- Diagonals II: player Easy→Hard sensitivity against Normal: +46.7 points.
- Diagonals III: player Easy→Hard sensitivity against Normal: +73.3 points.
- Diagonals III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV: player Easy→Hard sensitivity against Normal: +73.3 points.
- Diagonals IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams I: player Easy→Hard sensitivity against Normal: +66.7 points.
- Beams II: player Easy→Hard sensitivity against Normal: +46.7 points.
- Beams II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III: player Easy→Hard sensitivity against Normal: +70.0 points.
- Beams III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Beams III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Shields II: player Easy→Hard sensitivity against Normal: +36.7 points.
- Shields II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Shields III: player Easy→Hard sensitivity against Normal: +50.0 points.
- Challenge I: player Easy→Hard sensitivity against Normal: +63.3 points.
- Challenge II: player Easy→Hard sensitivity against Normal: +53.3 points.
- Challenge III: player Easy→Hard sensitivity against Normal: +56.7 points.
- Challenge III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV: player Easy→Hard sensitivity against Normal: +46.7 points.
- Challenge IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I: player Easy→Hard sensitivity against Normal: +80.0 points.
- Rite I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite II: player Easy→Hard sensitivity against Normal: +40.0 points.
- Rite III: player Easy→Hard sensitivity against Normal: +60.0 points.
- Rite III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV: player Easy→Hard sensitivity against Normal: +70.0 points.
- Ascension I: player Easy→Hard sensitivity against Normal: +70.0 points.
- Ascension I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II: player Easy→Hard sensitivity against Normal: +36.7 points.
- Ascension II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Side Step: player Easy→Hard sensitivity against Normal: +50.0 points.
