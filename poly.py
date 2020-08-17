from matrices import Matrix
import math
# class for working with polyhedra 
# poly stores a matrix of vertices, a list of pairs of vertex indices (to represent edges)
# needs to store faces too at some point, but we'll try those two things first 

class Poly:
	def __init__(self, v, e, f, c):
		self.verts = v
		self.edges = e
		self.faces = f
		self.center = c

	def translate(self, x, y, z):
		# should translate the center 
		self.verts = Matrix.translate(self.verts, x, y, z)
		self.center[0] += x
		self.center[1] += y
		self.center[2] += z

	def tr4nslate(self, x, y, z, w):
		self.verts = Matrix.tr4nslate(self.verts, x, y, z, w)
		self.center[0] += x
		self.center[1] += y
		self.center[2] += z
		self.center[3] += w

	def rotate(self, x, y, z):
		# translate so rotating around the origin
		self.verts = Matrix.translate(self.verts, -self.center[0], -self.center[1], -self.center[2])
		self.verts = Matrix.rotate(self.verts, x, y, z)
		self.verts = Matrix.translate(self.verts, self.center[0], self.center[1], self.center[2])

	def fortate(self, a, b, c, d, e, f):
		self.verts = Matrix.tr4nslate(self.verts, -self.center[0], -self.center[1], -self.center[2], -self.center[3])
		self.verts = Matrix.fortate(self.verts, a, b, c, d, e, f)
		self.verts = Matrix.tr4nslate(self.verts, self.center[0], self.center[1], self.center[2], self.center[3])

	def scale(self, *args):
		self.verts = Matrix.translate(self.verts, -self.center[0], -self.center[1], -self.center[2])
		self.verts = Matrix.scale(self.verts, args)
		self.verts = Matrix.translate(self.verts, self.center[0], self.center[1], self.center[2])

	def sc4le(self, *args):
		self.verts = Matrix.tr4nslate(self.verts, -self.center[0], -self.center[1], -self.center[2], -self.center[3])
		self.verts = Matrix.sc4le(self.verts, args)
		self.verts = Matrix.tr4nslate(self.verts, self.center[0], self.center[1], self.center[2], self.center[3])

class Cube(Poly):
	def __init__(self):
		self.verts=Matrix([[0.5,0.5,0.5],
			[0.5,0.5,-0.5],
			[0.5,-0.5,0.5],
			[-0.5,0.5,0.5],
			[0.5,-0.5,-0.5],
			[-0.5,-0.5,0.5],
			[-0.5,0.5,-0.5],
			[-0.5,-0.5,-0.5]])
		self.center=[0,0,0]
		self.edges=[[0,1],[0,2],[0,3],[1,4],
			[1,6],[2,4],[2,5],[3,5],
			[3,6],[4,7],[5,7],[6,7]]
		# should faces point to edges or vertices? I feel like vertices would be easier
		self.faces=[[0,1,3,6],[0,1,2,4],[0,2,3,5],
			[1,4,6,7],[2,4,5,7],[3,5,6,7]]

class Hypercube(Poly):
	def __init__(self):
		self.verts = Matrix([
			[0.5,0.5,0.5,0.5],#0
			[0.5,0.5,0.5,-0.5],#1
			[0.5,0.5,-0.5,0.5],#2
			[0.5,0.5,-0.5,-0.5],#3
			[0.5,-0.5,0.5,0.5],#4
			[0.5,-0.5,0.5,-0.5],#5
			[0.5,-0.5,-0.5,0.5],#6
			[0.5,-0.5,-0.5,-0.5],#7
			[-0.5,0.5,0.5,0.5],#8
			[-0.5,0.5,0.5,-0.5],#9
			[-0.5,0.5,-0.5,0.5],#10
			[-0.5,0.5,-0.5,-0.5],#11
			[-0.5,-0.5,0.5,0.5],#12
			[-0.5,-0.5,0.5,-0.5],#13
			[-0.5,-0.5,-0.5,0.5],#14
			[-0.5,-0.5,-0.5,-0.5]#15
			])
		self.center=[0,0,0,0]
		self.edges=[[0,1],[0,2],[0,4],[0,8],
			[1,3],[1,5],[1,9],[2,3],
			[2,6],[2,10],[3,7],[3,11],
			[4,5],[4,6],[4,12],[5,7],
			[5,13],[6,7],[6,14],[7,15],
			[8,9],[8,10],[8,12],[9,11],
			[9,13],[10,11],[10,14],[11,15],
			[12,13],[12,14],[13,15],[14,15]]
		# it's wrong but unnecessary
		self.faces=[[1,2,3,4]]

class FiveCell(Poly):
	def __init__(self):
		self.verts = Matrix([
			[1/math.sqrt(10), 1/math.sqrt(6), 1/math.sqrt(3),1],
			[1/math.sqrt(10), 1/math.sqrt(6), 1/math.sqrt(3),-1],
			[1/math.sqrt(10), 1/math.sqrt(6), -2/math.sqrt(3), 0],
			[1/math.sqrt(10), -math.sqrt(1.5), 0, 0],
			[-2*math.sqrt(2/5), 0, 0, 0]])

		self.center=[0,0,0,0]

		self.edges=[[0,1],[0,2],[0,3],[0,4],
			[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]

		self.faces=[[1,2,3]]

class Hexadecachoron(Poly):
	def __init__(self):
		self.verts = Matrix([
			[1,0,0,0],#0
			[0,1,0,0],#1
			[0,0,1,0],#2
			[0,0,0,1],#3
			[-1,0,0,0],#4
			[0,-1,0,0],#5
			[0,0,-1,0],#6
			[0,0,0,-1]])#7

		self.center=[0,0,0,0]

		self.edges=[
		[0,1],[0,2],[0,3],[0,5],
		[0,6],[0,7],[1,2],[1,3],
		[1,4],[1,6],[1,7],[2,3],
		[2,4],[2,5],[2,7],[3,4],
		[3,5],[3,6],[4,5],[4,6],
		[4,7],[5,6],[5,7],[6,7],]

		self.faces=[[1,2,3]]