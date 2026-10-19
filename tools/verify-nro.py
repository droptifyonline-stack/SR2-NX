#!/usr/bin/env python3
"""Check NRO segments, asset section, icon and NACP bounds. Not a hardware test."""
import argparse
import hashlib
from pathlib import Path
import struct


def verify(path):
    data = path.read_bytes()
    if len(data) < 0x80 or data[0x10:0x14] != b'NRO0':
        raise ValueError('Missing NRO0 header')
    size = struct.unpack_from('<I', data, 0x18)[0]
    if size < 0x80 or size + 0x38 > len(data) or data[size:size+4] != b'ASET':
        raise ValueError('Missing or invalid ASET section')
    for offset in (0x20, 0x28, 0x30):
        start, length = struct.unpack_from('<II', data, offset)
        if start + length > size:
            raise ValueError('Executable segment exceeds image')
    for name, offset in [('icon', 8), ('nacp', 24), ('romfs', 40)]:
        start, length = struct.unpack_from('<QQ', data, size + offset)
        if length and (start < 0x38 or size + start + length > len(data)):
            raise ValueError(f'{name} exceeds asset bounds')
        if name in ('icon', 'nacp') and not length:
            raise ValueError(f'Missing {name}')
        if name == 'nacp' and length != 0x4000:
            raise ValueError('Unexpected NACP size')
    return hashlib.sha256(data).hexdigest()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('nro', type=Path)
    args = parser.parse_args()
    try:
        digest = verify(args.nro)
    except (OSError, ValueError, struct.error) as error:
        parser.exit(1, f'NRO verification failed: {error}\n')
    print(f'{digest}  {args.nro.name}')
