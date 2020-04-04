from matrices import Matrix
# class for working with polyhedra 
# poly stores a matrix of vertices, a list of pairs of vertex indices (to represent edges)
# needs to store faces too at some point, but we'll try those two things first 

class Poly:
	def __init__(self, v, e, c):
		self.verts = v
		# do we pass the edges in or are we supposed to create them from the points? 
		# I don't think it's possible to figure out the edges from the points alone
		# which pairs are edges and which aren't? 
		# I think for a generic polyhedron these have to be inputed 
		# There will be specific polyhedra as well, such as cubes which are already solved 
		# polyhedra will need methods for manipulating verts, edges, and faces 
		# for now we assume that the edges are given correctly 
		self.edges = e
		# we need to know where the center is because that's where we're scaling, rotating from
		# center is just a list of numbers 
		self.center = c

	# the advantage of having edges be pointers is that we don't have to change them when transforming

	# we want methods for transforming polyhedra
	# we already have all the matrix methods, so all that needs doing is calling those on verts

	def translate(self, x, y, z):
		self.verts = Matrix.translate(self.verts, x, y, z)

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