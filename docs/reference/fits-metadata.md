# Reference: FITS Astrometric Metadata & Header Cards

The ESCAM G02 serializes uncompressed 16-bit linear sensor data into standard **FITS 4.0** containers formatted in 2880-byte padded logical blocks with 80-character header cards.

---

## Standard Primary Header Cards

| Keyword | Value Type | Description |
|---|---|---|
| `SIMPLE` | Logical (`T`) | Conforms to IAU standard FITS specification |
| `BITPIX` | Integer (`16`) | 16-bit unsigned integer pixel array |
| `NAXIS` | Integer (`2`) | Two-dimensional image matrix |
| `NAXIS1` | Integer (`1280`) | Image array width in pixels |
| `NAXIS2` | Integer (`720`) | Image array height in pixels |
| `BZERO` | Float (`32768.0`) | Offset for unsigned 16-bit mapping |
| `BSCALE` | Float (`1.0`) | Linear data scaling factor |
| `BAYERPAT`| String (`'RGGB'`) | Bayer color filter array pattern |
| `EXPTIME` | Float | Exposure duration in seconds |
| `DATAMIN` | Integer | Minimum pixel ADU value |
| `DATAMAX` | Integer | Maximum pixel ADU value |
| `DATE-OBS`| String | UTC ISO 8601 timestamp at start of exposure |
| `DATE-AVG`| String | UTC ISO 8601 timestamp at exposure midpoint |
| `INSTRUME`| String (`'ESCAM G02'`) | Sensor instrument model |
| `TELESCOP`| String (`'ESCAM Robotic Mount'`) | Integrated pan/tilt telescope mount |

---

## Astrometric WCS Header Cards (TAN Projection)

| Keyword | Value Type | Description |
|---|---|---|
| `RADESYS` | String (`'FK5'`) | Equatorial celestial reference frame |
| `EQUINOX` | Float (`2000.0`) | Equinox of celestial coordinates (J2000) |
| `WCSAXES` | Integer (`2`) | Number of celestial WCS axes |
| `CTYPE1`  | String (`'RA---TAN'`) | Gnomonic tangent plane projection for RA |
| `CTYPE2`  | String (`'DEC--TAN'`) | Gnomonic tangent plane projection for DEC |
| `CRPIX1`  | Float (`640.0`) | Optical center X reference pixel |
| `CRPIX2`  | Float (`360.0`) | Optical center Y reference pixel |
| `CRVAL1`  | Float | Right Ascension of reference pixel in degrees |
| `CRVAL2`  | Float | Declination of reference pixel in degrees |
| `CDELT1`  | Float | Pixel scale along RA axis (deg/pixel) |
| `CDELT2`  | Float | Pixel scale along DEC axis (deg/pixel) |
| `CROTA2`  | Float | Coordinate rotation angle in degrees |
| `SITELAT` | Float | Observatory site latitude in degrees North |
| `SITELONG`| Float | Observatory site longitude in degrees East |
