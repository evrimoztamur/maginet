# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 264 / 273 matchups complete, 7920 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-c440d9b67c0d7740`; bounded table capacity 32768 entries per search.

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

- Easy/Easy: W/L/D/U 30/0/0/0; mean 4.3 plies (3–17); 2314 nodes; node saturation 0.0%; fallbacks 0/130.

- Easy/Normal: W/L/D/U 30/0/0/0; mean 3.8 plies (3–13); 4806 nodes; node saturation 0.0%; fallbacks 0/114.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 4.1 plies (3–15); 20592 nodes; node saturation 0.0%; fallbacks 0/122.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 3.3 plies (3–9); 6811 nodes; node saturation 0.0%; fallbacks 0/98.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 3.0 plies (3–3); 8595 nodes; node saturation 0.0%; fallbacks 0/90.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 3.0 plies (3–3); 18048 nodes; node saturation 0.0%; fallbacks 0/90.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 6.4 plies (3–23); 49238 nodes; node saturation 0.0%; fallbacks 0/192.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 5.3 plies (3–17); 48358 nodes; node saturation 0.0%; fallbacks 0/158.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 5.6 plies (3–23); 72408 nodes; node saturation 0.0%; fallbacks 0/168.

## Basics II

Map (1, 0); distance 2.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 18/12/0/0; mean 7.1 plies (3–27); 4860 nodes; node saturation 0.0%; fallbacks 0/214.

- Easy/Normal: W/L/D/U 15/15/0/0; mean 4.7 plies (3–9); 12993 nodes; node saturation 0.0%; fallbacks 0/141.

- Easy/Hard: W/L/D/U 15/15/0/0; mean 6.6 plies (3–23); 100508 nodes; node saturation 0.0%; fallbacks 0/197.

- Normal/Easy: W/L/D/U 22/8/0/0; mean 4.3 plies (3–10); 17407 nodes; node saturation 0.0%; fallbacks 0/128.

- Normal/Normal: W/L/D/U 20/10/0/0; mean 4.1 plies (3–11); 23237 nodes; node saturation 0.0%; fallbacks 0/122.

- Normal/Hard: W/L/D/U 20/10/0/0; mean 4.8 plies (3–19); 107542 nodes; node saturation 0.0%; fallbacks 0/144.

- Hard/Easy: W/L/D/U 25/5/0/0; mean 6.0 plies (3–25); 233878 nodes; node saturation 0.0%; fallbacks 0/179.

- Hard/Normal: W/L/D/U 23/7/0/0; mean 5.4 plies (3–26); 238221 nodes; node saturation 0.0%; fallbacks 0/162.

- Hard/Hard: W/L/D/U 23/7/0/0; mean 4.9 plies (3–19); 289480 nodes; node saturation 0.0%; fallbacks 0/147.

## Basics III

Map (2, 0); distance 3.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 20/10/0/0; mean 16.2 plies (5–38); 14192 nodes; node saturation 0.0%; fallbacks 0/486.

- Easy/Normal: W/L/D/U 17/12/1/0; mean 13.9 plies (5–30); 63431 nodes; node saturation 0.0%; fallbacks 0/417.

- Easy/Hard: W/L/D/U 16/14/0/0; mean 8.6 plies (5–26); 698930 nodes; node saturation 0.8%; fallbacks 0/258.

- Normal/Easy: W/L/D/U 28/1/1/0; mean 10.7 plies (5–29); 75282 nodes; node saturation 0.0%; fallbacks 0/321.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 9.9 plies (5–22); 115892 nodes; node saturation 0.0%; fallbacks 0/296.

- Normal/Hard: W/L/D/U 23/7/0/0; mean 8.6 plies (5–29); 814332 nodes; node saturation 1.5%; fallbacks 0/259.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 15.9 plies (5–29); 1164527 nodes; node saturation 7.1%; fallbacks 0/476.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 20.8 plies (5–31); 1493780 nodes; node saturation 5.9%; fallbacks 0/623.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 13.3 plies (5–34); 2461685 nodes; node saturation 11.0%; fallbacks 0/399.

## Basics IV

Map (2, -1); distance 4.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 27/3/0/0; mean 19.6 plies (7–47); 13681 nodes; node saturation 0.0%; fallbacks 0/589.

- Easy/Normal: W/L/D/U 22/8/0/0; mean 18.4 plies (5–38); 53227 nodes; node saturation 0.0%; fallbacks 0/551.

- Easy/Hard: W/L/D/U 26/4/0/0; mean 19.6 plies (7–45); 718163 nodes; node saturation 0.0%; fallbacks 0/587.

- Normal/Easy: W/L/D/U 28/1/1/0; mean 14.9 plies (7–46); 57535 nodes; node saturation 0.0%; fallbacks 0/448.

- Normal/Normal: W/L/D/U 29/1/0/0; mean 13.2 plies (7–31); 88368 nodes; node saturation 0.0%; fallbacks 0/395.

- Normal/Hard: W/L/D/U 29/1/0/0; mean 11.6 plies (7–25); 604890 nodes; node saturation 0.0%; fallbacks 0/349.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 15.8 plies (7–35); 808015 nodes; node saturation 0.0%; fallbacks 0/474.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 14.2 plies (7–27); 919693 nodes; node saturation 0.0%; fallbacks 0/426.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 14.7 plies (7–29); 1401887 nodes; node saturation 0.0%; fallbacks 0/440.

## Patterns I

Map (3, -1); distance 5.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |

- Easy/Easy: W/L/D/U 13/17/0/0; mean 28.9 plies (16–58); 32996 nodes; node saturation 0.0%; fallbacks 0/867.

- Easy/Normal: W/L/D/U 11/18/1/0; mean 28.5 plies (14–53); 232125 nodes; node saturation 0.0%; fallbacks 0/854.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 28.6 plies (14–56); 3758635 nodes; node saturation 15.5%; fallbacks 0/858.

- Normal/Easy: W/L/D/U 24/6/0/0; mean 27.7 plies (19–50); 225636 nodes; node saturation 0.0%; fallbacks 0/832.

- Normal/Normal: W/L/D/U 19/9/2/0; mean 27.0 plies (17–50); 413436 nodes; node saturation 0.0%; fallbacks 0/810.

- Normal/Hard: W/L/D/U 5/25/0/0; mean 28.8 plies (17–44); 4474300 nodes; node saturation 18.6%; fallbacks 0/864.

- Hard/Easy: W/L/D/U 28/2/0/0; mean 31.8 plies (21–53); 4656148 nodes; node saturation 17.2%; fallbacks 0/955.

- Hard/Normal: W/L/D/U 28/1/1/0; mean 29.4 plies (17–51); 4748094 nodes; node saturation 19.4%; fallbacks 0/883.

- Hard/Hard: W/L/D/U 20/8/2/0; mean 30.2 plies (14–45); 8510828 nodes; node saturation 34.0%; fallbacks 0/905.

## Patterns II

Map (3, -2); distance 6.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |

- Easy/Easy: W/L/D/U 20/9/1/0; mean 43.8 plies (33–70); 41317 nodes; node saturation 0.0%; fallbacks 0/1313.

- Easy/Normal: W/L/D/U 6/23/1/0; mean 40.9 plies (23–61); 262553 nodes; node saturation 0.0%; fallbacks 0/1227.

- Easy/Hard: W/L/D/U 3/26/1/0; mean 37.6 plies (26–63); 6096229 nodes; node saturation 6.1%; fallbacks 0/1128.

- Normal/Easy: W/L/D/U 25/2/3/0; mean 42.9 plies (25–83); 242353 nodes; node saturation 0.0%; fallbacks 0/1286.

- Normal/Normal: W/L/D/U 20/9/1/0; mean 43.7 plies (29–78); 467315 nodes; node saturation 0.0%; fallbacks 0/1311.

- Normal/Hard: W/L/D/U 3/21/6/0; mean 43.6 plies (26–75); 7071298 nodes; node saturation 5.6%; fallbacks 0/1307.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 42.8 plies (25–67); 6484462 nodes; node saturation 4.1%; fallbacks 0/1285.

- Hard/Normal: W/L/D/U 25/0/5/0; mean 39.5 plies (25–75); 6242626 nodes; node saturation 2.4%; fallbacks 0/1185.

- Hard/Hard: W/L/D/U 9/5/16/0; mean 41.0 plies (29–85); 12442035 nodes; node saturation 4.9%; fallbacks 0/1229.

## Patterns III

Map (4, -2); distance 7.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% |

- Easy/Easy: W/L/D/U 15/12/3/0; mean 43.7 plies (29–65); 87960 nodes; node saturation 0.0%; fallbacks 0/1310.

- Easy/Normal: W/L/D/U 10/16/4/0; mean 47.9 plies (34–69); 838135 nodes; node saturation 0.0%; fallbacks 0/1436.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 41.4 plies (26–63); 8701912 nodes; node saturation 29.4%; fallbacks 0/1241.

- Normal/Easy: W/L/D/U 27/3/0/0; mean 41.6 plies (27–64); 873433 nodes; node saturation 0.0%; fallbacks 0/1248.

- Normal/Normal: W/L/D/U 20/7/3/0; mean 47.1 plies (25–90); 1791335 nodes; node saturation 0.0%; fallbacks 0/1412.

- Normal/Hard: W/L/D/U 10/19/1/0; mean 43.4 plies (21–68); 9833823 nodes; node saturation 28.7%; fallbacks 0/1303.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 44.8 plies (27–84); 9970612 nodes; node saturation 30.1%; fallbacks 0/1344.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 45.2 plies (27–71); 10374644 nodes; node saturation 29.1%; fallbacks 0/1355.

- Hard/Hard: W/L/D/U 19/8/3/0; mean 43.3 plies (31–67); 17762362 nodes; node saturation 55.6%; fallbacks 0/1300.

## Diagonals I

Map (5, -2); distance 8.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 10.1 plies (5–14); 5810 nodes; node saturation 0.0%; fallbacks 0/304.

- Easy/Normal: W/L/D/U 15/15/0/0; mean 10.8 plies (5–17); 25363 nodes; node saturation 0.0%; fallbacks 0/323.

- Easy/Hard: W/L/D/U 13/17/0/0; mean 13.0 plies (5–28); 263224 nodes; node saturation 0.0%; fallbacks 0/389.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 7.8 plies (5–15); 27792 nodes; node saturation 0.0%; fallbacks 0/235.

- Normal/Normal: W/L/D/U 29/1/0/0; mean 8.3 plies (5–15); 45633 nodes; node saturation 0.0%; fallbacks 0/249.

- Normal/Hard: W/L/D/U 27/3/0/0; mean 9.6 plies (5–26); 282219 nodes; node saturation 0.0%; fallbacks 0/287.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 14.9 plies (5–29); 424414 nodes; node saturation 0.0%; fallbacks 0/446.

- Hard/Normal: W/L/D/U 27/3/0/0; mean 16.3 plies (5–33); 497366 nodes; node saturation 0.0%; fallbacks 0/489.

- Hard/Hard: W/L/D/U 27/3/0/0; mean 14.4 plies (5–33); 802126 nodes; node saturation 0.0%; fallbacks 0/431.

## Diagonals II

Map (5, -1); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 14/15/1/0; mean 40.4 plies (24–74); 54676 nodes; node saturation 0.0%; fallbacks 0/1211.

- Easy/Normal: W/L/D/U 6/21/3/0; mean 39.0 plies (22–57); 375496 nodes; node saturation 0.0%; fallbacks 0/1169.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 37.6 plies (18–77); 6867760 nodes; node saturation 21.0%; fallbacks 0/1129.

- Normal/Easy: W/L/D/U 25/4/1/0; mean 36.7 plies (21–53); 386004 nodes; node saturation 0.0%; fallbacks 0/1102.

- Normal/Normal: W/L/D/U 16/13/1/0; mean 42.2 plies (24–78); 720566 nodes; node saturation 0.0%; fallbacks 0/1265.

- Normal/Hard: W/L/D/U 3/26/1/0; mean 35.9 plies (24–56); 6996051 nodes; node saturation 20.8%; fallbacks 0/1078.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 39.7 plies (23–64); 7257556 nodes; node saturation 22.3%; fallbacks 0/1192.

- Hard/Normal: W/L/D/U 22/5/3/0; mean 40.1 plies (21–80); 6979668 nodes; node saturation 19.0%; fallbacks 0/1204.

- Hard/Hard: W/L/D/U 24/5/1/0; mean 37.0 plies (21–65); 12839164 nodes; node saturation 38.3%; fallbacks 0/1111.

## Diagonals III

Map (5, 0); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% |

- Easy/Easy: W/L/D/U 14/14/2/0; mean 46.0 plies (29–82); 86689 nodes; node saturation 0.0%; fallbacks 0/1379.

- Easy/Normal: W/L/D/U 5/21/4/0; mean 46.3 plies (28–74); 704429 nodes; node saturation 0.0%; fallbacks 0/1389.

- Easy/Hard: W/L/D/U 2/26/2/0; mean 45.0 plies (32–72); 8882314 nodes; node saturation 25.4%; fallbacks 0/1350.

- Normal/Easy: W/L/D/U 20/10/0/0; mean 44.0 plies (25–74); 716438 nodes; node saturation 0.0%; fallbacks 0/1319.

- Normal/Normal: W/L/D/U 14/15/1/0; mean 46.6 plies (21–83); 1257609 nodes; node saturation 0.0%; fallbacks 0/1397.

- Normal/Hard: W/L/D/U 9/17/4/0; mean 44.3 plies (28–79); 8877498 nodes; node saturation 23.7%; fallbacks 0/1330.

- Hard/Easy: W/L/D/U 27/2/1/0; mean 43.6 plies (27–65); 9012451 nodes; node saturation 28.4%; fallbacks 0/1307.

- Hard/Normal: W/L/D/U 26/3/1/0; mean 41.9 plies (25–67); 8956112 nodes; node saturation 26.7%; fallbacks 0/1257.

- Hard/Hard: W/L/D/U 18/11/1/0; mean 42.9 plies (21–63); 15140243 nodes; node saturation 47.8%; fallbacks 0/1286.

## Diagonals IV

Map (4, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |

- Easy/Easy: W/L/D/U 10/17/3/0; mean 57.6 plies (35–104); 105289 nodes; node saturation 0.0%; fallbacks 0/1728.

- Easy/Normal: W/L/D/U 4/24/2/0; mean 55.6 plies (28–96); 863374 nodes; node saturation 0.0%; fallbacks 0/1667.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 53.4 plies (35–82); 12777865 nodes; node saturation 31.9%; fallbacks 0/1602.

- Normal/Easy: W/L/D/U 23/6/1/0; mean 55.3 plies (27–84); 977877 nodes; node saturation 0.0%; fallbacks 0/1659.

- Normal/Normal: W/L/D/U 15/9/6/0; mean 52.0 plies (35–81); 1795053 nodes; node saturation 0.0%; fallbacks 0/1560.

- Normal/Hard: W/L/D/U 2/19/9/0; mean 54.9 plies (35–94); 13154475 nodes; node saturation 31.8%; fallbacks 0/1647.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 52.6 plies (35–77); 12830502 nodes; node saturation 34.0%; fallbacks 0/1578.

- Hard/Normal: W/L/D/U 23/3/4/0; mean 50.9 plies (29–85); 13247482 nodes; node saturation 34.9%; fallbacks 0/1527.

- Hard/Hard: W/L/D/U 13/12/5/0; mean 53.6 plies (35–80); 23917427 nodes; node saturation 63.6%; fallbacks 0/1609.

## Beams I

Map (6, -2); distance 9.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 14/16/0/0; mean 16.9 plies (9–29); 16403 nodes; node saturation 0.0%; fallbacks 0/508.

- Easy/Normal: W/L/D/U 8/22/0/0; mean 16.1 plies (9–34); 83045 nodes; node saturation 0.0%; fallbacks 0/482.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 17.9 plies (12–28); 2017343 nodes; node saturation 2.2%; fallbacks 0/537.

- Normal/Easy: W/L/D/U 23/6/1/0; mean 15.6 plies (9–31); 84867 nodes; node saturation 0.0%; fallbacks 0/468.

- Normal/Normal: W/L/D/U 19/11/0/0; mean 14.7 plies (9–28); 147052 nodes; node saturation 0.0%; fallbacks 0/441.

- Normal/Hard: W/L/D/U 21/9/0/0; mean 14.1 plies (9–32); 1777619 nodes; node saturation 3.1%; fallbacks 0/423.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 17.4 plies (9–33); 1793078 nodes; node saturation 6.5%; fallbacks 0/522.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 16.0 plies (11–43); 1927658 nodes; node saturation 7.3%; fallbacks 0/481.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 18.1 plies (9–31); 3704767 nodes; node saturation 6.8%; fallbacks 0/544.

## Beams II

Map (6, -3); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 10/18/2/0; mean 59.7 plies (35–129); 77635 nodes; node saturation 0.0%; fallbacks 0/1790.

- Easy/Normal: W/L/D/U 5/24/1/0; mean 55.7 plies (30–118); 529231 nodes; node saturation 0.0%; fallbacks 0/1672.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 47.8 plies (24–91); 9037471 nodes; node saturation 23.4%; fallbacks 0/1434.

- Normal/Easy: W/L/D/U 19/6/5/0; mean 52.1 plies (21–87); 521791 nodes; node saturation 0.0%; fallbacks 0/1564.

- Normal/Normal: W/L/D/U 11/11/8/0; mean 57.7 plies (36–106); 1009816 nodes; node saturation 0.0%; fallbacks 0/1731.

- Normal/Hard: W/L/D/U 1/26/3/0; mean 50.7 plies (35–97); 9849258 nodes; node saturation 21.3%; fallbacks 0/1521.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 45.3 plies (29–75); 8993060 nodes; node saturation 24.8%; fallbacks 0/1360.

- Hard/Normal: W/L/D/U 21/2/7/0; mean 53.4 plies (35–101); 10877444 nodes; node saturation 24.5%; fallbacks 0/1602.

- Hard/Hard: W/L/D/U 14/8/8/0; mean 50.7 plies (35–79); 17917944 nodes; node saturation 38.4%; fallbacks 0/1522.

## Beams III

Map (6, -4); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |

- Easy/Easy: W/L/D/U 13/14/3/0; mean 37.5 plies (20–76); 69478 nodes; node saturation 0.0%; fallbacks 0/1124.

- Easy/Normal: W/L/D/U 4/26/0/0; mean 30.8 plies (18–48); 434067 nodes; node saturation 0.0%; fallbacks 0/925.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 36.1 plies (16–70); 8351102 nodes; node saturation 30.9%; fallbacks 0/1084.

- Normal/Easy: W/L/D/U 21/4/5/0; mean 40.2 plies (18–62); 698927 nodes; node saturation 0.0%; fallbacks 0/1207.

- Normal/Normal: W/L/D/U 13/16/1/0; mean 36.3 plies (17–55); 881053 nodes; node saturation 0.0%; fallbacks 0/1090.

- Normal/Hard: W/L/D/U 1/27/2/0; mean 36.7 plies (22–62); 8908331 nodes; node saturation 30.2%; fallbacks 0/1100.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 36.1 plies (19–66); 8924409 nodes; node saturation 31.9%; fallbacks 0/1083.

- Hard/Normal: W/L/D/U 26/4/0/0; mean 34.1 plies (17–55); 8566883 nodes; node saturation 32.1%; fallbacks 0/1024.

- Hard/Hard: W/L/D/U 12/17/1/0; mean 37.5 plies (19–61); 17465631 nodes; node saturation 61.7%; fallbacks 0/1126.

## Shields I

Map (7, -2); distance 10.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 30/0/0/0; mean 6.7 plies (5–13); 2794 nodes; node saturation 0.0%; fallbacks 0/202.

- Easy/Normal: W/L/D/U 30/0/0/0; mean 6.7 plies (5–15); 8623 nodes; node saturation 0.0%; fallbacks 0/202.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 6.5 plies (5–17); 30119 nodes; node saturation 0.0%; fallbacks 0/196.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 5.3 plies (5–7); 7170 nodes; node saturation 0.0%; fallbacks 0/160.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 6.1 plies (5–17); 12266 nodes; node saturation 0.0%; fallbacks 0/182.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 5.9 plies (5–9); 29539 nodes; node saturation 0.0%; fallbacks 0/176.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 7.7 plies (5–17); 49755 nodes; node saturation 0.0%; fallbacks 0/232.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 8.9 plies (5–25); 65142 nodes; node saturation 0.0%; fallbacks 0/266.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 8.6 plies (5–21); 97170 nodes; node saturation 0.0%; fallbacks 0/258.

## Shields II

Map (7, -1); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Hard | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |

- Easy/Easy: W/L/D/U 5/24/1/0; mean 26.4 plies (18–51); 36795 nodes; node saturation 0.0%; fallbacks 0/793.

- Easy/Normal: W/L/D/U 4/26/0/0; mean 24.6 plies (16–47); 253207 nodes; node saturation 0.0%; fallbacks 0/739.

- Easy/Hard: W/L/D/U 1/29/0/0; mean 27.0 plies (18–40); 5648563 nodes; node saturation 27.1%; fallbacks 0/811.

- Normal/Easy: W/L/D/U 9/19/2/0; mean 32.0 plies (18–50); 264054 nodes; node saturation 0.0%; fallbacks 0/961.

- Normal/Normal: W/L/D/U 5/23/2/0; mean 29.1 plies (16–61); 508676 nodes; node saturation 0.0%; fallbacks 0/874.

- Normal/Hard: W/L/D/U 1/28/1/0; mean 31.7 plies (18–52); 5767047 nodes; node saturation 20.3%; fallbacks 0/952.

- Hard/Easy: W/L/D/U 19/9/2/0; mean 32.0 plies (16–58); 6187477 nodes; node saturation 25.2%; fallbacks 0/961.

- Hard/Normal: W/L/D/U 15/13/2/0; mean 32.6 plies (16–54); 6225296 nodes; node saturation 24.2%; fallbacks 0/977.

- Hard/Hard: W/L/D/U 3/24/3/0; mean 34.1 plies (16–54); 11736538 nodes; node saturation 45.3%; fallbacks 0/1024.

## Shields III

Map (7, 0); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 16/14/0/0; mean 21.7 plies (13–32); 25113 nodes; node saturation 0.0%; fallbacks 0/650.

- Easy/Normal: W/L/D/U 8/21/1/0; mean 21.7 plies (12–58); 165485 nodes; node saturation 0.0%; fallbacks 0/650.

- Easy/Hard: W/L/D/U 4/26/0/0; mean 21.1 plies (11–42); 3168124 nodes; node saturation 18.0%; fallbacks 0/632.

- Normal/Easy: W/L/D/U 20/4/6/0; mean 21.4 plies (13–38); 159486 nodes; node saturation 0.0%; fallbacks 0/642.

- Normal/Normal: W/L/D/U 17/11/2/0; mean 20.8 plies (11–33); 287136 nodes; node saturation 0.0%; fallbacks 0/624.

- Normal/Hard: W/L/D/U 9/19/2/0; mean 20.7 plies (15–34); 3056538 nodes; node saturation 16.4%; fallbacks 0/622.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 19.4 plies (11–31); 3505246 nodes; node saturation 23.3%; fallbacks 0/583.

- Hard/Normal: W/L/D/U 23/3/4/0; mean 20.7 plies (13–33); 3860752 nodes; node saturation 23.5%; fallbacks 0/620.

- Hard/Hard: W/L/D/U 23/6/1/0; mean 17.3 plies (11–36); 5551931 nodes; node saturation 38.7%; fallbacks 0/520.

## Challenge I

Map (8, -4); distance 13.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |

- Easy/Easy: W/L/D/U 13/17/0/0; mean 16.5 plies (12–33); 20431 nodes; node saturation 0.0%; fallbacks 0/496.

- Easy/Normal: W/L/D/U 5/24/1/0; mean 17.4 plies (12–28); 140324 nodes; node saturation 0.0%; fallbacks 0/521.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 21.4 plies (11–38); 2971948 nodes; node saturation 16.7%; fallbacks 0/642.

- Normal/Easy: W/L/D/U 20/9/1/0; mean 19.0 plies (11–34); 140991 nodes; node saturation 0.0%; fallbacks 0/570.

- Normal/Normal: W/L/D/U 11/13/6/0; mean 18.2 plies (12–29); 274552 nodes; node saturation 0.0%; fallbacks 0/547.

- Normal/Hard: W/L/D/U 3/25/2/0; mean 19.0 plies (12–37); 3147950 nodes; node saturation 20.3%; fallbacks 0/571.

- Hard/Easy: W/L/D/U 28/1/1/0; mean 20.2 plies (13–37); 3102517 nodes; node saturation 18.9%; fallbacks 0/607.

- Hard/Normal: W/L/D/U 24/3/3/0; mean 21.2 plies (11–39); 3471288 nodes; node saturation 21.7%; fallbacks 0/637.

- Hard/Hard: W/L/D/U 16/8/6/0; mean 18.5 plies (12–31); 6361352 nodes; node saturation 45.5%; fallbacks 0/556.

## Challenge II

Map (8, -5); distance 14.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% |

- Easy/Easy: W/L/D/U 15/13/2/0; mean 32.3 plies (16–53); 30181 nodes; node saturation 0.0%; fallbacks 0/968.

- Easy/Normal: W/L/D/U 7/21/2/0; mean 28.5 plies (13–50); 171411 nodes; node saturation 0.0%; fallbacks 0/856.

- Easy/Hard: W/L/D/U 2/26/2/0; mean 24.4 plies (12–56); 3482433 nodes; node saturation 15.6%; fallbacks 0/733.

- Normal/Easy: W/L/D/U 20/6/4/0; mean 26.1 plies (11–57); 158054 nodes; node saturation 0.0%; fallbacks 0/783.

- Normal/Normal: W/L/D/U 13/13/4/0; mean 28.2 plies (10–77); 282058 nodes; node saturation 0.0%; fallbacks 0/845.

- Normal/Hard: W/L/D/U 5/24/1/0; mean 24.9 plies (12–46); 3491632 nodes; node saturation 16.0%; fallbacks 0/746.

- Hard/Easy: W/L/D/U 27/2/1/0; mean 26.5 plies (15–48); 3591358 nodes; node saturation 15.6%; fallbacks 0/794.

- Hard/Normal: W/L/D/U 25/4/1/0; mean 28.7 plies (13–76); 3261618 nodes; node saturation 13.6%; fallbacks 0/862.

- Hard/Hard: W/L/D/U 15/14/1/0; mean 33.1 plies (14–63); 7870579 nodes; node saturation 28.7%; fallbacks 0/993.

## Challenge III

Map (8, -6); distance 15.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Hard | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |

- Easy/Easy: W/L/D/U 10/18/2/0; mean 57.0 plies (39–80); 119698 nodes; node saturation 0.0%; fallbacks 0/1711.

- Easy/Normal: W/L/D/U 0/29/1/0; mean 50.8 plies (28–84); 1023887 nodes; node saturation 0.1%; fallbacks 0/1524.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 51.0 plies (35–100); 12117907 nodes; node saturation 33.0%; fallbacks 0/1531.

- Normal/Easy: W/L/D/U 25/5/0/0; mean 54.3 plies (33–93); 1043154 nodes; node saturation 0.0%; fallbacks 0/1630.

- Normal/Normal: W/L/D/U 9/19/2/0; mean 56.4 plies (34–76); 1911255 nodes; node saturation 0.0%; fallbacks 0/1693.

- Normal/Hard: W/L/D/U 0/30/0/0; mean 51.1 plies (35–76); 13179006 nodes; node saturation 33.4%; fallbacks 0/1533.

- Hard/Easy: W/L/D/U 25/3/2/0; mean 54.2 plies (37–77); 12833320 nodes; node saturation 32.9%; fallbacks 0/1625.

- Hard/Normal: W/L/D/U 16/12/2/0; mean 54.6 plies (35–82); 13613076 nodes; node saturation 33.2%; fallbacks 0/1638.

- Hard/Hard: W/L/D/U 7/23/0/0; mean 52.4 plies (34–88); 22698828 nodes; node saturation 59.9%; fallbacks 0/1571.

## Challenge IV

Map (9, -6); distance 16.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |

- Easy/Easy: W/L/D/U 9/15/6/0; mean 61.9 plies (42–92); 111477 nodes; node saturation 0.0%; fallbacks 0/1857.

- Easy/Normal: W/L/D/U 1/28/1/0; mean 64.6 plies (42–107); 1049512 nodes; node saturation 0.0%; fallbacks 0/1939.

- Easy/Hard: W/L/D/U 0/30/0/0; mean 56.5 plies (42–89); 14640202 nodes; node saturation 35.9%; fallbacks 0/1695.

- Normal/Easy: W/L/D/U 20/5/5/0; mean 68.4 plies (43–116); 1006000 nodes; node saturation 0.0%; fallbacks 0/2053.

- Normal/Normal: W/L/D/U 13/13/4/0; mean 66.8 plies (50–95); 2207974 nodes; node saturation 0.0%; fallbacks 0/2003.

- Normal/Hard: W/L/D/U 2/23/5/0; mean 60.4 plies (35–88); 15499572 nodes; node saturation 33.4%; fallbacks 0/1811.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 58.9 plies (37–87); 14786137 nodes; node saturation 35.0%; fallbacks 0/1767.

- Hard/Normal: W/L/D/U 17/4/9/0; mean 64.2 plies (39–86); 15841474 nodes; node saturation 32.3%; fallbacks 0/1925.

- Hard/Hard: W/L/D/U 12/14/4/0; mean 65.7 plies (36–106); 30215880 nodes; node saturation 62.3%; fallbacks 0/1970.

## Rite I

Map (8, -2); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% | 3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3% |
| Normal | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% |

- Easy/Easy: W/L/D/U 13/14/3/0; mean 63.7 plies (36–101); 151226 nodes; node saturation 0.0%; fallbacks 0/1910.

- Easy/Normal: W/L/D/U 3/27/0/0; mean 55.9 plies (36–82); 1524309 nodes; node saturation 1.3%; fallbacks 0/1676.

- Easy/Hard: W/L/D/U 1/29/0/0; mean 58.5 plies (34–82); 14035929 nodes; node saturation 33.8%; fallbacks 0/1756.

- Normal/Easy: W/L/D/U 27/2/1/0; mean 51.8 plies (29–105); 1562111 nodes; node saturation 1.9%; fallbacks 0/1555.

- Normal/Normal: W/L/D/U 15/11/4/0; mean 64.8 plies (40–98); 3073365 nodes; node saturation 3.0%; fallbacks 0/1944.

- Normal/Hard: W/L/D/U 10/18/2/0; mean 54.9 plies (36–94); 15186887 nodes; node saturation 39.2%; fallbacks 0/1646.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 52.8 plies (35–89); 12841042 nodes; node saturation 34.1%; fallbacks 0/1584.

- Hard/Normal: W/L/D/U 25/5/0/0; mean 53.6 plies (41–73); 13905540 nodes; node saturation 37.7%; fallbacks 0/1607.

- Hard/Hard: W/L/D/U 21/9/0/0; mean 57.6 plies (32–110); 26783561 nodes; node saturation 67.1%; fallbacks 0/1728.

## Rite II

Map (9, -2); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% |

- Easy/Easy: W/L/D/U 16/12/2/0; mean 20.1 plies (12–47); 18613 nodes; node saturation 0.0%; fallbacks 0/602.

- Easy/Normal: W/L/D/U 8/22/0/0; mean 17.3 plies (12–31); 109211 nodes; node saturation 0.0%; fallbacks 0/520.

- Easy/Hard: W/L/D/U 5/25/0/0; mean 19.8 plies (12–30); 2795233 nodes; node saturation 8.3%; fallbacks 0/593.

- Normal/Easy: W/L/D/U 22/8/0/0; mean 17.8 plies (13–32); 122616 nodes; node saturation 0.0%; fallbacks 0/534.

- Normal/Normal: W/L/D/U 18/12/0/0; mean 18.9 plies (14–27); 210614 nodes; node saturation 0.0%; fallbacks 0/566.

- Normal/Hard: W/L/D/U 11/19/0/0; mean 21.2 plies (15–30); 3254149 nodes; node saturation 11.3%; fallbacks 0/635.

- Hard/Easy: W/L/D/U 26/4/0/0; mean 22.5 plies (14–43); 3561795 nodes; node saturation 12.0%; fallbacks 0/676.

- Hard/Normal: W/L/D/U 20/10/0/0; mean 22.5 plies (14–41); 3653767 nodes; node saturation 11.6%; fallbacks 0/674.

- Hard/Hard: W/L/D/U 10/20/0/0; mean 22.4 plies (14–37); 6948767 nodes; node saturation 29.0%; fallbacks 0/672.

## Rite III

Map (9, -3); distance 13.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |

- Easy/Easy: W/L/D/U 13/15/2/0; mean 49.9 plies (29–81); 90189 nodes; node saturation 0.0%; fallbacks 0/1498.

- Easy/Normal: W/L/D/U 6/23/1/0; mean 44.6 plies (28–66); 813942 nodes; node saturation 0.0%; fallbacks 0/1338.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 43.7 plies (26–76); 9747886 nodes; node saturation 31.1%; fallbacks 0/1312.

- Normal/Easy: W/L/D/U 25/5/0/0; mean 43.1 plies (25–80); 756262 nodes; node saturation 0.0%; fallbacks 0/1292.

- Normal/Normal: W/L/D/U 15/8/7/0; mean 43.3 plies (23–75); 1494479 nodes; node saturation 0.2%; fallbacks 0/1299.

- Normal/Hard: W/L/D/U 5/23/2/0; mean 41.6 plies (25–68); 9620051 nodes; node saturation 28.4%; fallbacks 0/1249.

- Hard/Easy: W/L/D/U 28/0/2/0; mean 43.7 plies (23–69); 10145963 nodes; node saturation 32.8%; fallbacks 0/1312.

- Hard/Normal: W/L/D/U 23/5/2/0; mean 45.5 plies (25–83); 10670232 nodes; node saturation 30.0%; fallbacks 0/1366.

- Hard/Hard: W/L/D/U 7/19/4/0; mean 51.3 plies (29–93); 21397499 nodes; node saturation 57.7%; fallbacks 0/1540.

## Rite IV

Map (9, -4); distance 14.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 21/7/2/0; mean 26.6 plies (16–53); 30298 nodes; node saturation 0.0%; fallbacks 0/798.

- Easy/Normal: W/L/D/U 4/25/1/0; mean 26.4 plies (14–53); 218106 nodes; node saturation 0.0%; fallbacks 0/792.

- Easy/Hard: W/L/D/U 4/25/1/0; mean 28.2 plies (16–46); 3824138 nodes; node saturation 16.4%; fallbacks 0/845.

- Normal/Easy: W/L/D/U 29/0/1/0; mean 21.5 plies (13–41); 221597 nodes; node saturation 0.0%; fallbacks 0/644.

- Normal/Normal: W/L/D/U 23/6/1/0; mean 29.7 plies (17–55); 417492 nodes; node saturation 0.0%; fallbacks 0/890.

- Normal/Hard: W/L/D/U 12/16/2/0; mean 27.1 plies (15–55); 4009130 nodes; node saturation 17.7%; fallbacks 0/814.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 24.0 plies (15–41); 4104582 nodes; node saturation 22.2%; fallbacks 0/720.

- Hard/Normal: W/L/D/U 28/1/1/0; mean 25.8 plies (17–45); 4017821 nodes; node saturation 19.6%; fallbacks 0/775.

- Hard/Hard: W/L/D/U 24/3/3/0; mean 30.5 plies (17–64); 8169329 nodes; node saturation 32.4%; fallbacks 0/914.

## Ascension I

Map (10, -4); distance 15.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |

- Easy/Easy: W/L/D/U 16/11/3/0; mean 60.8 plies (37–84); 136408 nodes; node saturation 0.0%; fallbacks 0/1825.

- Easy/Normal: W/L/D/U 4/23/3/0; mean 54.8 plies (36–86); 1189065 nodes; node saturation 0.0%; fallbacks 0/1645.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 51.3 plies (36–80); 13608792 nodes; node saturation 39.9%; fallbacks 0/1540.

- Normal/Easy: W/L/D/U 25/3/2/0; mean 58.1 plies (35–102); 1318193 nodes; node saturation 0.3%; fallbacks 0/1742.

- Normal/Normal: W/L/D/U 16/10/4/0; mean 59.9 plies (37–88); 2630171 nodes; node saturation 1.3%; fallbacks 0/1796.

- Normal/Hard: W/L/D/U 5/22/3/0; mean 57.0 plies (36–91); 15150145 nodes; node saturation 35.9%; fallbacks 0/1711.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 55.0 plies (37–78); 14479116 nodes; node saturation 38.8%; fallbacks 0/1650.

- Hard/Normal: W/L/D/U 27/2/1/0; mean 59.1 plies (39–93); 15421073 nodes; node saturation 34.4%; fallbacks 0/1772.

- Hard/Hard: W/L/D/U 16/12/2/0; mean 61.4 plies (43–91); 28939608 nodes; node saturation 66.7%; fallbacks 0/1843.

## Ascension II

Map (11, -4); distance 16.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |

- Easy/Easy: W/L/D/U 10/18/2/0; mean 79.8 plies (41–140); 271301 nodes; node saturation 0.0%; fallbacks 0/2394.

- Easy/Normal: W/L/D/U 2/25/3/0; mean 85.9 plies (42–126); 3072241 nodes; node saturation 4.0%; fallbacks 0/2577.

- Easy/Hard: W/L/D/U 0/29/1/0; mean 71.8 plies (41–111); 20404955 nodes; node saturation 45.0%; fallbacks 0/2154.

- Normal/Easy: W/L/D/U 27/1/2/0; mean 78.2 plies (43–117); 2922271 nodes; node saturation 2.8%; fallbacks 0/2347.

- Normal/Normal: W/L/D/U 11/16/3/0; mean 81.5 plies (42–143); 5582328 nodes; node saturation 7.9%; fallbacks 0/2445.

- Normal/Hard: W/L/D/U 2/27/1/0; mean 81.2 plies (62–139); 24816746 nodes; node saturation 46.1%; fallbacks 0/2436.

- Hard/Easy: W/L/D/U 26/1/3/0; mean 70.1 plies (41–107); 19488602 nodes; node saturation 43.8%; fallbacks 0/2103.

- Hard/Normal: W/L/D/U 16/7/7/0; mean 81.5 plies (61–111); 24299398 nodes; node saturation 43.0%; fallbacks 0/2445.

- Hard/Hard: W/L/D/U 7/21/2/0; mean 83.4 plies (54–115); 44410827 nodes; node saturation 81.6%; fallbacks 0/2501.

## Crossfire

Map (7, -4); distance 12.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% |

- Easy/Easy: W/L/D/U 29/0/1/0; mean 2.5 plies (1–26); 1084 nodes; node saturation 0.0%; fallbacks 0/75.

- Easy/Normal: W/L/D/U 29/0/1/0; mean 2.5 plies (1–26); 2448 nodes; node saturation 0.0%; fallbacks 0/75.

- Easy/Hard: W/L/D/U 29/0/1/0; mean 2.4 plies (1–26); 10331 nodes; node saturation 0.0%; fallbacks 0/71.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 1.5 plies (1–7); 3531 nodes; node saturation 0.0%; fallbacks 0/44.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 1.5 plies (1–7); 3945 nodes; node saturation 0.0%; fallbacks 0/44.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 1.4 plies (1–5); 5140 nodes; node saturation 0.0%; fallbacks 0/42.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 4.1 plies (1–15); 44072 nodes; node saturation 0.0%; fallbacks 0/123.

- Hard/Normal: W/L/D/U 27/3/0/0; mean 4.4 plies (1–11); 48456 nodes; node saturation 0.0%; fallbacks 0/131.

- Hard/Hard: W/L/D/U 27/3/0/0; mean 3.6 plies (1–11); 49651 nodes; node saturation 0.0%; fallbacks 0/107.

## Side Step

Map (6, 0); distance 11.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% |
| Normal | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% |

- Easy/Easy: W/L/D/U 12/6/12/0; mean 15.7 plies (3–32); 8395 nodes; node saturation 0.0%; fallbacks 0/472.

- Easy/Normal: W/L/D/U 15/6/9/0; mean 15.0 plies (3–28); 31253 nodes; node saturation 0.0%; fallbacks 0/450.

- Easy/Hard: W/L/D/U 11/18/1/0; mean 12.3 plies (3–23); 155134 nodes; node saturation 0.0%; fallbacks 0/368.

- Normal/Easy: W/L/D/U 26/1/3/0; mean 12.2 plies (3–37); 30278 nodes; node saturation 0.0%; fallbacks 0/365.

- Normal/Normal: W/L/D/U 25/2/3/0; mean 10.3 plies (3–23); 40073 nodes; node saturation 0.0%; fallbacks 0/310.

- Normal/Hard: W/L/D/U 20/8/2/0; mean 11.3 plies (3–23); 163870 nodes; node saturation 0.0%; fallbacks 0/340.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 15.1 plies (3–23); 212809 nodes; node saturation 0.0%; fallbacks 0/452.

- Hard/Normal: W/L/D/U 29/0/1/0; mean 9.9 plies (3–23); 167781 nodes; node saturation 0.0%; fallbacks 0/298.

- Hard/Hard: W/L/D/U 25/3/2/0; mean 11.4 plies (3–23); 295409 nodes; node saturation 0.0%; fallbacks 0/343.

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

- Normal/Easy: W/L/D/U 30/0/0/0; mean 8.7 plies (7–15); 13677 nodes; node saturation 0.0%; fallbacks 0/260.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 10.2 plies (7–19); 82295 nodes; node saturation 0.0%; fallbacks 0/306.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.

- Basics I → Basics II: 33.3-point drop; supported candidate.
- Basics II → Basics III: -20.0-point drop; below spike threshold.
- Basics III → Basics IV: -10.0-point drop; below spike threshold.
- Basics IV → Patterns I: 33.3-point drop; supported candidate.
- Patterns I → Patterns II: -3.3-point drop; below spike threshold.
- Patterns II → Patterns III: 0.0-point drop; below spike threshold.
- Patterns III → Diagonals I: -30.0-point drop; below spike threshold.
- Diagonals I → Beams I: 33.3-point drop; supported candidate.
- Beams I → Shields I: -36.7-point drop; below spike threshold.
- Shields I → Rite I: 50.0-point drop; supported candidate.
- Rite I → Rite II: -10.0-point drop; below spike threshold.
- Rite II → Rite III: 10.0-point drop; below spike threshold.
- Rite III → Rite IV: -26.7-point drop; below spike threshold.
- Rite IV → Ascension I: 23.3-point drop; followup candidate.
- Ascension I → Ascension II: 16.7-point drop; below spike threshold.

### Optional route 1 (dead end)

- Diagonals I → Diagonals II: 43.3-point drop; supported candidate.
- Diagonals II → Diagonals III: 6.7-point drop; below spike threshold.
- Diagonals III → Diagonals IV: -3.3-point drop; below spike threshold.

### Optional route 2 (dead end)

- Beams I → Beams II: 26.7-point drop; followup candidate.
- Beams II → Beams III: -6.7-point drop; below spike threshold.

### Optional route 3 (dead end)

- Beams III → Crossfire: -56.7-point drop; below spike threshold.
- Crossfire → Challenge I: 63.3-point drop; supported candidate.

### Optional route 4 (dead end)

- Diagonals III → Side Step: -36.7-point drop; below spike threshold.
- Side Step → Shields III: 26.7-point drop; followup candidate.

### Optional route 5 (dead end)


### Optional route 6 (dead end)

- Shields I → Shields II: 83.3-point drop; supported candidate.
- Shields II → Shields III: -40.0-point drop; below spike threshold.

### Optional route 7 (dead end)

- Rite IV → Challenge I: 40.0-point drop; supported candidate.
- Challenge I → Challenge II: -6.7-point drop; below spike threshold.
- Challenge II → Challenge III: 13.3-point drop; below spike threshold.
- Challenge III → Challenge IV: -13.3-point drop; below spike threshold.

## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Basics II: player Easy→Hard sensitivity against Normal: +26.7 points.
- Basics III: player Easy→Hard sensitivity against Normal: +43.3 points.
- Basics IV: player Easy→Hard sensitivity against Normal: +26.7 points.
- Patterns I: player Easy→Hard sensitivity against Normal: +56.7 points.
- Patterns II: player Easy→Hard sensitivity against Normal: +63.3 points.
- Patterns III: player Easy→Hard sensitivity against Normal: +63.3 points.
- Patterns III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals I: player Easy→Hard sensitivity against Normal: +40.0 points.
- Diagonals II: player Easy→Hard sensitivity against Normal: +53.3 points.
- Diagonals III: player Easy→Hard sensitivity against Normal: +70.0 points.
- Diagonals III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV: player Easy→Hard sensitivity against Normal: +63.3 points.
- Diagonals IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Diagonals IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Beams I: player Easy→Hard sensitivity against Normal: +70.0 points.
- Beams II: player Easy→Hard sensitivity against Normal: +53.3 points.
- Beams III: player Easy→Hard sensitivity against Normal: +73.3 points.
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
- Challenge II: player Easy→Hard sensitivity against Normal: +60.0 points.
- Challenge III: player Easy→Hard sensitivity against Normal: +53.3 points.
- Challenge III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV: player Easy→Hard sensitivity against Normal: +53.3 points.
- Challenge IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Challenge IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I: player Easy→Hard sensitivity against Normal: +73.3 points.
- Rite I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite II: player Easy→Hard sensitivity against Normal: +40.0 points.
- Rite III: player Easy→Hard sensitivity against Normal: +56.7 points.
- Rite III Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite III Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV: player Easy→Hard sensitivity against Normal: +80.0 points.
- Ascension I: player Easy→Hard sensitivity against Normal: +76.7 points.
- Ascension I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II: player Easy→Hard sensitivity against Normal: +46.7 points.
- Ascension II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Ascension II Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Side Step: player Easy→Hard sensitivity against Normal: +46.7 points.
