<body style="background-color:rgb(16, 23, 38);">
<div style="color:white">

## Documentation

# Table of Contents
1. [Vector](#Vector)
2. [Matrix](#Matrix)

<a name="Vector"></a>
### Vector

```
struct Vector {
    pub x: f64
    pub y: f64
    pub z: f64
}
```

#### **`new(x, y, z)`**
- in: `(f64, f64, f64)`
- out: `Vector` <br />
Basic constructor

#### **`from_points(p1, p2)`**
- in: `(&Point, &Point)`
- out: `Vector` <br />
Creates a vector from two points. First given point is treated as the beginning point of a created vector.

#### **`dot(&self, other)`**
- in `Vector`
- out `f64`<br />
Calculate the dot product of this and a given vector.

#### **`cross(&self, other)`**
- in `Vector`
- out `Vector`<br />
Calculate the cross product of this and a given vector. The returned vector is one that is perpendicular to both of the given vectors.

#### **`angle(&self, other)`**
- in `Vector`
- out `f64`<br />
Calculate the angle between this and a given vector. Returned value is in radians. 

#### **`length(&self)`**
- in `-`
- out `f64`<br />
Calculate the length of a vector.

#### **`normalize(&self)`**
- in `-`
- out `Vector`<br />
Normalize a vector, meaning it's length will become equal to 1.

#### **`to_string(&self)`**
- in `-`
- out `String`
Write values of a vector to a string. Formatting is: `[x, y, z]`

#### **`+ operator`**
Performs the + operation on two vectors

#### **`- operator`**
Performs the - operation on two vectors

#### **`* operator`**
Performs the *+* operation between vector and a scalar.<br />
For example: `Vector * 2.0` or `Vector * 0.3`

<a name="Matrix"></a>
### Matrix


</span>