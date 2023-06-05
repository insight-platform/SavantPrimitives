from savant_rs.primitives import VideoObject, AttributeValue, Attribute
from savant_rs.primitives.geometry import RBBox
from timeit import default_timer as timer

o = VideoObject(
    id=1,
    creator="some",
    label="person",
    bbox=RBBox(0.1, 0.2, 0.3, 0.4, None),
    confidence=0.5,
    attributes={},
    track=None,
)

t = timer()

bts = bytes(256)

a = Attribute(creator="other", name="attr", values=[
    # Value.bytes(dims=[8, 3, 8, 8], blob=bts, confidence=None),
    AttributeValue.integer(1, confidence=0.5),
    AttributeValue.integer(2, confidence=0.5),
    AttributeValue.integer(3, confidence=0.5),
    AttributeValue.integer(4, confidence=0.5),
])

for _ in range(1_000):
    o.set_attribute(a)
    a = o.get_attribute(creator="other", name="attr")
    # x = a.name

print(timer() - t)
