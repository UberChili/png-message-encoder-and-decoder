
# The Idea
This is an interesting project usually suggested to people looking into interesting stuff to do for a reinforced learning of the Rust Programming Language. I found it suggested somewhere online, saw it and thought was somewhat hard and above my level, but the author states that programmers that get those feelings are precisely the intended _audience_ of his book, and that everything would make sense later down the road. Seems fun!

The first three chapters have us implementing a basic **PNG file**. PNG files are essentially just a list of *chunks*, each containing their own data. Each *chunk* has a *type* that can be represented as a **4 character string**. There are standard chunk types for things like image data, but there's no rule that would prevent us from inserting our own chunks with whatever data we want, without breaking the PNG file itself. We can even tell PNG decoders to ignore our chunks, depending on how we capitalize our chunk types.

# Part 1: Chunk Types

## Some preparations
I decided to use the **[Anyhow](https://crates.io/crates/anyhow)** crate. Which provides **anyhow::Error**, a trait object based error type for easy idiomatic error handling in Rust applications.

for now, our **main** function is pretty simple, but in preparation and because of **Anyhow**, I added the following alisases:
```rust
pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;
```
This will make things easier for us down the road, using idiomatic and more elegant errorr handling.

With that done, we can begin. We will be working on *Chunk Types*, thus, we will implement our own. These are pretty easy since they're essentially just 4 alphabetic characters. Although our *Chunk Types* should always be **valid** Chunks (more on this later), it should not be possible to construct an invalid chunk type using our public interface.

The [PNG File Structure Spec](https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) will explain to us what a **valid Chunk Type** looks like.

## Chunk naming conventions
You can read more about the specifics of how this works by visiting the PNG File Structure Spec, mentioned above. Otherwise this article would be incredibly long. The important thing bere to mention is that we will be working with a connsecutive sequence of characters to form a Chunk Type and that we need to be sure it is valid, following the conventions. But of course, we have to treat these characters as bytes. In Rust, a byte can be represented as an **u8**. So we can also represent a vector or an array of bytes as, for example, a [u8]. A valid Chunk Type is formed by four characters exactly, and the capitalization of each character in the "string" has specific meanings.
<label for="mn-string" class="margin-toggle">&#8853;</label>
<input type="checkbox" id="mn-string" class="margin-toggle"/>
<span class="marginnote">
    Of course, this can be considered a *string*, as it is a sequence of bytes that can be represented as characters. But for this, it is helpful to think of it more as just a sequence of bytes.
</span>

So, our **Chunk Type** type would look like the following:
```rust
pub struct ChunkType {
    chunk_type: [u8; 4],
}
```

## Some trait implementations
And first, we should implement the **FromStr** trait for our Chunk type, which will allow us to get a new Chunk Type from an **str**, it will end up looking like the following:
```rust
impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        if s.len() != 4 {
            return Err(anyhow!(
                "Incorrect lengh of string {} for Chunk Type. Has to be of length 4.",
                s
            ));
        }

        let mut code_vec: Vec<u8> = vec![];

        for i in s.chars() {
            if !i.is_ascii_alphabetic() {
                return Err(anyhow!("Incorrect char for Chunk Type: {}", i));
            } else {
                code_vec.push(i as u8);
            }
        }

        let code = match <[u8; 4]>::try_from(code_vec) {
            Ok(val) => val,
            Err(_err) => {
                return Err(anyhow!(
                    "Error: . Could not convert s ({}) into array of bytes",
                    s
                ));
            }
        };
        Ok(ChunkType { chunk_type: code })
    }
}
```
A little lengthy although I think this is good code.
The main points are understanding that we of course expect a **&str** that is 4 in length and then we check if all of those characters are valid **ascii alphabetic** characters. Right now, we do not need to validate if the chunk type is valid, which would involve checking whether the characters are upper case or lower case, but that is not required right now as that is validated later.

Next, we do the implementations of **TryFrom<[u8; 4]>** and **Display** for **Chunk Type**, both look like the following.
```rust
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { chunk_type: value })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.chunk_type {
            write!(f, "{}", c as char)?;
        }

        Ok(())
    }
}
```

## How these traits work
**TryFrom<[u8; 4]>** is deliberately minimal. 
The function signature guarantees we get exactly four bytes (thanks to the type system), so no length checking is needed. Full validation of the chunk type rules is handled separately in **is_valid()**.

This follows Rust's conventions. These are **conversion traits**, not full constructors with validation. Their job is to turn raw data into the struct if the shape is correct (right length, character type). Also considering flexibility and separation of concerns.

## Validation methods
Next, we implement the following methods for **Chunk Type**:
```rust
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }

    pub fn is_valid(&self) -> bool {
        if self.chunk_type.len() != 4 {
            return false;
        }
        for c in self.chunk_type {
            if !c.is_ascii_alphabetic() {
                return false;
            }
        }
        if !self.is_reserved_bit_valid() {
            return false;
        }
        true
    }

    pub fn is_critical(&self) -> bool {
        let bit5 = (self.chunk_type[0] & (1 << 5)) != 0;
        if bit5 {
            return false;
        }
        true
    }

    pub fn is_public(&self) -> bool {
        let bit5 = (self.chunk_type[1] & (1 << 5)) != 0;
        if bit5 {
            return false;
        }
        true
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let bit5 = (self.chunk_type[2] & (1 << 5)) != 0;
        if bit5 {
            return false;
        }
        true
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let bit5 = (self.chunk_type[3] & (1 << 5)) != 0;
        if bit5 {
            return true;
        }
        false
    }
}
```
These methods are what actually do the structure validation to make sure we get a correct Chunk Type, particularly **is_valid()**.

With this, we can run our tests doing **cargo test**. I am not showing the contents of the tests right here, as it would take a lot of space, but you can check them in the Github repo. And after testing, we get output like the following:

```bash
cargo test
   Compiling pngme v0.1.0 (/home/andros/Programming/Rust/pngme)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src/main.rs (/home/andros/Programming/Rust/pngme/target/debug/deps/pngme-bf6281b14950262c)

running 14 tests
test chunk_type::tests::test_chunk_type_from_bytes ... ok
test chunk_type::tests::test_chunk_type_from_str ... ok
test chunk_type::tests::test_chunk_type_is_critical ... ok
test chunk_type::tests::test_chunk_type_is_not_critical ... ok
test chunk_type::tests::test_chunk_type_is_not_public ... ok
test chunk_type::tests::test_chunk_type_is_public ... ok
test chunk_type::tests::test_chunk_type_is_reserved_bit_invalid ... ok
test chunk_type::tests::test_chunk_type_is_reserved_bit_valid ... ok
test chunk_type::tests::test_chunk_type_is_safe_to_copy ... ok
test chunk_type::tests::test_chunk_type_is_unsafe_to_copy ... ok
test chunk_type::tests::test_chunk_type_string ... ok
test chunk_type::tests::test_chunk_type_trait_impls ... ok
test chunk_type::tests::test_invalid_chunk_is_valid ... ok
test chunk_type::tests::test_valid_chunk_is_valid ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Pretty.
Of course, I wanted to show the output of some unit tests here but I'll mostly omit them for the rest of the devlog. Again, if you want to see them you can check them out in the Github repo of this project. There was unit tests for every major part of this project:
- chunk_type.rs
- chunk.rs
- png.rs

# Part 2: Chunks
Now that we have our **Chunk Type** type working correctly, we can move on to form some Chunks. This was one of the trickiest parts since we need to actually read and work with some varieble-length data.

Our **Chunk** type would look like the following:
```rust
pub struct Chunk {
    lenght: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
```

ADD A MARGIN IMAGE HERE
(Or maybe not necesarily as margin image, just normally)

Once again, we will be needing the [PNG File Structure](https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html).
