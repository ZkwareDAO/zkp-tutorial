import time
import hashlib

# finite field of secp256k1:
a = 0
b = 7
F = FiniteField (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F)
# Elliptic Curve defined by y^2 = x^3 + 0x + 7 over finite field F ( = secp256k1)
E = EllipticCurve ([F(0), F(7)]) 
G = E(55066263022277343669578718895168534326250603453777594175500187360389116729240, 32670510020758816978083085130507043184471273380659243275938904335757337482424)
P = E(69335761065767984070318781108127416310968753866933119760392423089576366173459, 113425617697416972613102767146321902225172329004525144463444008550345431352693)
n = E.order()
h = 1
Fn = FiniteField(n)

def digest(msg):
	msg = str(msg)
	return Integer('0x' + hashlib.sha1(msg.encode()).hexdigest())

# Algorithm Elliptic curve key pair generation
# Require:
#	generator point P of elliptic curve E
#	order n of P and the field Zn defined by n
# Input:
#	N/A
# Output:
#	keypair (Q, d)
#		public key point Q on curve E
#		private key d in [1, n-1]
#

def ec_keygen():
	d = randint(1, n - 1)
	Q = d * P
	return (Q, d)

# Algorithm signature generation
# Require:
#	generator point P of elliptic curve E
#	order n of P and the field Zn defined by n
# Input:
#	message m
#	private key d in [1, n - 1]
# Output:
#	signature (r, s) where r, s in Zn
#
# 1. Generate random k in 0<k<n;
# 2. Calculate point Q of EC, Q=k*P;
# 3. Consider r = x_Q mod n, where x_Q — x-coord of point Q. If r=0, go to step 1;
# 4. Calculate digest: e=digest(m);
# 5. Calculate s = (rd+ke) mod n. if s=0, go to step 1;
#
def ecdsa_sign(d, m):
	r = 0
	s = 0
	while s == 0:
		k = 1
		while r == 0:
			k = randint(1, n - 1)
			Q = k * P
			(x1, y1) = Q.xy()
			r = Fn(x1)
		kk = Fn(k)
		e = digest(m)
		s = kk ^ (-1) * (e + d * r)
	return [r, s]

# Algorithm signature verification
# Require:
#	generator point P of curve E
#	order n of P and the field Zn defined by n
# Input:
#	public key point Q on curve E
#	message m
#	signature sig = (r, s) where r, s in Zn
# Output:
#	True or False
#
# 1. Calculate digest: e=digest(m);
# 2. Calculate w = s-1 mod n;
# 3. Calculate u1 = e*w mod n and u2 = r*w mod n;
# 4. Calculate point X = u1*P + u2*Q;
# 5. Consider R = x_X mod n, where x_X — x-coord of X;
# 6. If v=r, then its verified.
#

def ecdsa_verify(Q, m, r, s):
	e = digest(m)
	w = s ^ (-1)
	u1 = (e * w)
	u2 = (r * w)
	P1 = Integer(u1) * P
	P2 = Integer(u2) * Q
	X = P1 + P2
	(x, y) = X.xy()
	v = Fn(x)
	return v == r

# TEST

(Q, d) = ec_keygen()
m = 'signed message'
not_m = 'signed message'

[r, s] = ecdsa_sign(d, m)
result = ecdsa_verify(Q, not_m, r, s)

print ("EC Public Key       : ", Q.xy())
print ("EC Private Key      : ", d)
print ("Signed Message      : ", m)
print ("Signature     : ")
print (" r = ", r)
print (" s = ", s)
print ("Verified Message    : ", not_m)
print ("Verification Result : ", result)

t0 = time.time()
for _ in range(1000):
    [r, s] = ecdsa_sign(d, m)
    result = ecdsa_verify(Q, not_m, r, s)

t1 = time.time()
print(t1-t0)
