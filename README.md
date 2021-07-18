# steganos
steganos is a simple program to hide a message in an image
or to extract a message from such an image, respectively.

## basic concept
The concept is anything but special, it's a simple steganographic task.
Each bit of a text message is hidden in one of the RGB values of a pixel in an image.
Each pixel has one byte (eight bit) for the red value, one for green and one for blue.
Therefore one pixel can store three bits if only one bit is manipulated.<br>

Example: We want to hide the text `a` in a picture.
```
[255, 255, 255]	# this is the first pixel, it's white.

# the RGB values as bits look as follows:
1 1 1 1 1 1 1 1  |  1 1 1 1 1 1 1 1  |  1 1 1 1 1 1 1 1

# the first character of our message is 'a', in bits:
0 1 1 0 0 0 0 1

# now we hide the first bit of our first character in the first value of our image, that's the red value of the first pixel
# so that you can't see the difference between the original and the encrypted image, we take the least significant bit
1 1 1 1 1 1 1 1 # this is the original R value
1 1 1 1 1 1 1 0 # the 0 is the first bit from the 'a' we want to hide

# next we can go on with the second bit of the 'a' and hide it inside the green value
1 1 1 1 1 1 1 1 # original G value
1 1 1 1 1 1 1 1 # because the second bit of 'a' is 1 the manipulated bit stays 1

# we proceed according to this scheme and hide all bits of our text 'a'
```

## encryption with a key
steganos further complicates this hiding process by encrypting it with a key.
The key consists of another string of text.
This text, too, is read as bits.
Always when something is hidden in a RGB value of a text the program looks for the next bit of the key.
If it's `0` everything is normal and the message bit is hidden in the least significant bit of the current RGB value.
But if the current key bit is 1 the program not only manipulates the last bit,
it hides two bits. One in the second least significant bit, one in the least significant bit.<br>

Example: The current pixel is again `[255, 255, 255]`, the next pixel is `[115, 0, 0]`,
the current message character to hide is again `a` and
the current key character is also `a`.
```
# first pixel
[255, 255, 255] -> 	1 1 1 1 1 1 1 1  |  1 1 1 1 1 1 1 1  | 1 1 1 1 1 1 1 1

# message
'a' -> 	0 1 1 0 0 0 0 1

# key
'a' -> 	0 1 1 0 0 0 0 1

# first bit: key is 0, so everything is normal
1 1 1 1 1 1 1 1 # original value (red)
1 1 1 1 1 1 1 0 # the 0 from the 'a' causes the 0 in the last bit

# second bit: key is 1, so the program hides two bits
1 1 1 1 1 1 1 1 # original value (green)
1 1 1 1 1 1 1 1 # the last two 1s come from the 'a', in this case nothing changes

# third bit: key is 1 again
1 1 1 1 1 1 1 1 # original value (blue)
1 1 1 1 1 1 0 0 # the first 0, 1 and 1 from our letter 'a' are already hidden; because the key is 1 the next two bits are hidden

# fourth bit: key is 0 again. The fourth value is hidden in the second pixel. In our example it's a red pixel ([115, 0, 0])
0 1 1 1 0 0 1 1 # the red value of the second pixel (115)
0 1 1 1 0 0 1 0 # the next bit of the 'a' is 0; key is 0, so only one bit to hide
```

## end of message (EOM)
steganos cycles through the given text strings.
So if you hide a short message, nevertheless every pixel of the image is manipulated.
The output of the decrypt function is therefore as long as the image is big.
To prevent outputting a way to big repetitive message when you decrypt,
you can specify a EOM (end of message) identifier. This is a pattern to declare the message ended.
The default value is `*[END]*`.
Now you can put this identifier inside of your message and steganos will only output what is before this pattern.

## tests
There are [tests](tests/) included in this repo.
The encrypted files are encrypted using the key `This is the key.`
and the message hidden in the images is `This is the message.*[END]*`.
