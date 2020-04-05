import math

class Matrix:
	def __init__(self, data):
		# elements of a matrix are vectors 
		self.data = data
		self.col = len(data)
		self.row = len(data[0])
		for x in data:
			if len(x) != self.row:
				print "Matrix dimensions inconsistent"

	def display(self):
		for i in range(self.row):
			row = ""
			for j in range(self.col):
				row += str(self.data[j][i]) + " "
			print row

	# what would I use the determinant for? 
	# the determinant can be used for finding inverses 
	def determinant(self):
		if self.row == self.col:
			if self.row == 2:
				return (self.data[0][0]*self.data[1][1])-(self.data[0][1]*self.data[1][0])
			elif self.row == 3:
				print "Need to put in"
			else:
				print "I can't be bothered to write the code to calculate the determinants of larger matrices"
		else:
			print "Can't find determinant of non square matrix"

	# this seems to work
	@staticmethod
	def mtimes(A, B):
		if isinstance(B, Matrix):
			if A.col != B.row:
				print "Cannot multiply these matrices"
				return
			C = Matrix.zeros([A.row, B.col])
			for i in range(A.row):
				for j in range(B.col):
					for k in range(A.col):
						C.data[j][i] += A.data[k][i]*B.data[j][k]
			return C
		elif isinstance(B, list):
			if A.col != len(B):
				print "Cannot multiply these matrices"
				return
			C = Matrix.zeros([A.row, 1]) 
			for i in range(A.row):
				for k in range(A.col):
					C.data[0][i] += A.data[k][i]*B[k]
			return C
		else: print "Incorrect argument types"

	# expect a matrix with three rows 
	@staticmethod
	def rotate(A, x, y, z):
		xRotation = Matrix([[1,0,0],[0,math.cos(x),math.sin(x)],[0,-math.sin(x),math.cos(x)]])
		yRotation = Matrix([[math.cos(y),0,-math.sin(y)],[0,1,0],[math.sin(y),0,math.cos(y)]])
		zRotation = Matrix([[math.cos(z),math.sin(z),0],[-math.sin(z),math.cos(z),0],[0,0,1]])
		rotation = Matrix.mtimes(zRotation,Matrix.mtimes(yRotation,xRotation))
		return Matrix.mtimes(rotation,A)

	# expect a matrix with three rows 
	@staticmethod
	def translate(A, x, y, z):
		B = A
		for i in range(B.col):
			B.data[i][0]+=x
			B.data[i][1]+=y 
			B.data[i][2]+=z
		return B

	@staticmethod
	def scale(A, s):
		if len(s)==3:
			x=s[0]
			y=s[1]
			z=s[2]
		elif len(s)==1:
			x=s[0]
			y=x
			z=x
		B = A
		for i in range(B.col):
			B.data[i][0]*=x
			B.data[i][1]*=y
			B.data[i][2]*=z
		return B

	# this seems to work
	@staticmethod
	def zeros(dim):
		matrix = []
		for j in range(dim[1]):
			matrix.append([])
			for i in range(dim[0]):
				matrix[j].append(0)
		return Matrix(matrix)

	# this seems to work 
	@staticmethod
	def identity(dim):
		matrix = []
		for i in range(dim):
			matrix.append([])
			for j in range(dim):
				if i==j:
					matrix[i].append(1)
				else:
					matrix[i].append(0)
		return Matrix(matrix)