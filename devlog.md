
# The Idea
This is an interesting project usually suggested to people looking into interesting stuff to do for a reinforced learning of the Rust Programming Language. I found it suggested somewhere online, saw it and thought was somewhat hard and above my level, but the author states that programmers that get those feelings are precisely the intended _audience_ of his book, and that everything would make sense later down the road. Seems fun!

The first three chapters have us implementing a basic **PNG file**. PNG files are essentially just a list of *chunks*, each containing their own data. Each *chunk* has a *type* that can be represented as a **4 character string**. There are standard chunk types for things like image data, but there's no rule that would prevent us from inserting our own chunks with whatever data we want, without breaking the PNG file itself. We can even tell PNG decoders to ignore our chunks, depending on how we capitalize our chunk types.

# Part 1: Chunk Types
First of all, we will be working on *Chunk Types*, thus, we will implement our own. These are pretty easy since they're essentially just 4 alphabetic characters. Although our *Chunk Types* should always be **valid** Chunks (more on this later), it should not be possible to construct an invalid chunk type using our public interface.

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
