# DAO-Governance-Room
Here there is a concept of basic structure for the Polyhedron, implementation can be improved with more time but it works, the important is the structure and the algorithm to enumerate the tiles.
<br>Aim:
<li> –	Preserve position and adjacent tiles when possible
<li> –	Preserve distance from pentagons also if the polyhedron expands
<li>–	Best possible granularity: you can chose any polyhedron size, both even and odd
<li> –	Keep things symmetric when possible
Structure:
–	The Goldberg polyhedron has 12 pentagons numerated from 0 to 11, each pentagon is linked to his 5 neighbors
–	Each pentagon has 5 sectors, each sector is identified by the numbers of the 3 corner pentagons
–	Each sectors (owned by a pentagon) don’t contains all the hexagons of that triangular area but only the ones owned by the current pentagon. The algorithm to split the hexagons is described after.
–	Note that with this structure sectors are repeated 3 times (one for each corner pentagon). But hexagons are never repeated. Each sector is actually containing a third of the whole sector (maybe we should rename them “sector parts ”).
Enumeration:
Each tile is identified from the pentagon that owns it (0-11), the sector of the pentagon it is into (0-5), number of left steps from the pentagon and number of right steps .
note that each hexagon has 3 or more identification possible following this method but only one with a fixed pentagon.
How do we chose the pentagon:
1.	shortest path is better
2.	path with more left steps is better
3.	pentagon with lower id is better
Those rules are enough to grant unique tag for each hexagon. If we expand the polyhedron the same tags will stay in the same position relative to the pentagons and new tags will pop up in the tiles far away from the pentagons.
Run the program to list all the tags from a polyhedron of given size.
P. S. Sectors are created firs and then divided in 3 to facilitate the process of finding the adjacent tiles.
