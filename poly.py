from matrices import Matrix
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

	def rotate(self, x, y, z):
		# translate so rotating around the origin
		self.verts = Matrix.translate(self.verts, -self.center[0], -self.center[1], -self.center[2])
		self.verts = Matrix.rotate(self.verts, x, y, z)
		self.verts = Matrix.translate(self.verts, self.center[0], self.center[1], self.center[2])

	def scale(self, *args):
		self.verts = Matrix.translate(self.verts, -self.center[0], -self.center[1], -self.center[2])
		self.verts = Matrix.scale(self.verts, args)
		self.verts = Matrix.translate(self.verts, self.center[0], self.center[1], self.center[2])

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