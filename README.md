# batch-report-probe

A batch job. It writes `out/report.csv` and `out/summary.json`, then exits 0.
It is not a web server and produces nothing servable — which is the point:
it exists to verify that a platform does not mistake "ran to completion" for
"rendered a website".
