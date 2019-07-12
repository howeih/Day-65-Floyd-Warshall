Day 65: Floyd-Warshall
<br>
We initialize the solution matrix same as the input graph matrix as a first step. Then we update the solution matrix by considering all vertices as an intermediate vertex. The idea is to one by one pick all vertices and updates all shortest paths which include the picked vertex as an intermediate vertex in the shortest path. When we pick vertex number k as an intermediate vertex, we already have considered vertices {0, 1, 2, .. k-1} as intermediate vertices. For every pair (i, j) of the source and destination vertices respectively, there are two possible cases.
<br>
1) k is not an intermediate vertex in shortest path from i to j. We keep the value of dist[i][j] as it is.
<br>
2) k is an intermediate vertex in shortest path from i to j. We update the value of dist[i][j] as dist[i][k] + dist[k][j] if dist[i][j] > dist[i][k] + dist[k][j]
<br>
The following figure shows the above optimal substructure property in the all-pairs shortest path problem.
<br>
![Alt text](floyd_warshall.jpg?raw=true "floyd_warshall")
<br>
graph:
![Alt text](graph.png?raw=true "graph")
<br>
result:
```

start:
-----
    0    2    6    4
  INF    0    3  INF
    7  INF    0    1
    5  INF   12    0

result:
-----
    0    2    5    4
    9    0    3    4
    6    8    0    1
    5    7   10    0

```