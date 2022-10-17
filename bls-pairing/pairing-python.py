from curve import (
    curve_order,
    G1,
    G2,
    pairing,
)
from curve.encoding import (
    encodePubKey,
    decodePubKey,
    encodePrivKey,
    decodePrivKey,
    encodeSignature,
    decodeSignature,
    ENDIANNESS
)
import random
import time

from hashlib import sha256
#print(hex(curve_order), G1, G2)

# Represents a message as a point which belongs to the eliptic curve
# Simplified version (probably insecure)
def hashToPoint(message):
    # TODO secure hashing function
    hint = int.from_bytes(sha256(message).digest(), byteorder=ENDIANNESS)
    h = hint % curve_order
    return G2 * h


# Generates both public and secret keys
def keyGen():
    sk = random.randint(0, curve_order)
    pk = G1 * sk
    return pk, sk


# Generates a signature of a file
def sign(message, privKey):
    H = hashToPoint(message)
    signature = privKey * H
    return signature


# Checks the signature of a file
def verify(msg, sig, pubKey):
    H = hashToPoint(msg)
    p1 = pairing(pubKey, H)
    p2 = pairing(G1, sig)
    return p1 == p2

pk, sk = keyGen()
m = "helloworld"
s = sign(m.encode(), sk)
h = hashToPoint(m.encode())
#print(h)
#print(pairing(pk, h))
#print(pairing(G1, s))
#print(s)
#print(verify(m.encode(), s, pk))

t0 = time.time()
for _ in range(10):
    pairing(pk, h)

t1 = time.time()
print(t1-t0)