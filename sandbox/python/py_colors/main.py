import colorsys
import subprocess

def copyclip(text):
    subprocess.run("clip", text=True, input=text)
    

def hsl_to_rgb(h: float, s: float, l: float) -> tuple[int, int, int]:
    """
    Converts HSL (Hue, Saturation, Lightness) to RGB.
    
    Args:
        h (float): Hue value, typically from 0.0 to 360.0.
        s (float): Saturation value, typically from 0.0 to 1.0.
        l (float): Lightness value, typically from 0.0 to 1.0.
        
    Returns:
        tuple[int, int, int]: A tuple containing the Red, Green, and Blue values,
                               each ranging from 0 to 255.
    """
    # Normalize h to be within the range [0.0, 1.0] for the colorsys module
    h_normalized = h / 360.0
    
    # colorsys.hls_to_rgb expects h, l, s in the range [0.0, 1.0]
    # Note: colorsys uses HLS, not HSL. The order is different!
    # HSL (h, s, l) -> HLS (h, l, s)
    r_float, g_float, b_float = colorsys.hls_to_rgb(h_normalized, l, s)
    
    # Scale the float values from [0.0, 1.0] to [0, 255] and convert to integers
    r_int = int(r_float * 255)
    g_int = int(g_float * 255)
    b_int = int(b_float * 255)
    
    return (r_int, g_int, b_int)

def rgb_to_hex(r: int, g: int, b: int) -> str:
    """
    Converts RGB values to a hexadecimal color code string.
    
    Args:
        r (int): Red value from 0 to 255.
        g (int): Green value from 0 to 255.
        b (int): Blue value from 0 to 255.
        
    Returns:
        str: A hexadecimal color code string, e.g., "#RRGGBB".
    """
    # Format each integer as a two-digit hexadecimal string and concatenate
    return f"#{r:02x}{g:02x}{b:02x}"

if __name__ == "__main__":

    #  76  26  26 #4C1A1A


    # Example 1: A bright orange
    
    light_gray   = [100, 90, 75, 60, 45, 30, 15, 0]
    light_colors = [80, 70, 60, 50, 40, 30, 20]
    
    # gray
    text = []
    for l in light_gray:
        r, g, b = hsl_to_rgb(0, 0, l/100)
        hex = rgb_to_hex(r,g,b)

        msg = f'{r} {g} {b} {hex}'
        text.append(msg)
        print(msg)
    
    colors = [5, 50, 150, 200, 280, 336]
    for c in colors:
        for l in light_colors:
            r, g, b = hsl_to_rgb(c, 0.5, l/100)
            hex = rgb_to_hex(r,g,b)

            msg = f'{r} {g} {b} {hex}'
            text.append(msg)
            print(msg)
    
    # copy
    copyclip('\n'.join(text))
