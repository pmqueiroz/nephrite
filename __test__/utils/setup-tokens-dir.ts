import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';

export async function setupTokensDir() {
  const ostmpdir = os.tmpdir();
  const tmpdir = path.join(ostmpdir, 'nephrite-test-');
  const folderPath = await fs.mkdtemp(tmpdir);
  await fs.mkdir(path.join(folderPath, 'src', 'tokens'), { recursive: true });
  await fs.writeFile(
    path.join(folderPath, 'src', 'tokens', 'button.tokens.json'),
    JSON.stringify(
      {
        button: {
          primary: {
            background: {
              value: '#007bff',
              type: 'color',
            },
            color: {
              value: '#ffffff',
              type: 'color',
            },
          },
          secondary: {
            background: {
              value: '#6c757d',
              type: 'color',
            },
            color: {
              value: '#ffffff',
              type: 'color',
            },
          },
        },
      },
      null,
      2,
    ),
  );

  return folderPath;
}
