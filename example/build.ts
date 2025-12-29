import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { Nephrit, NephritLogLevel, TransformKind } from '../index.js';

const tempDir = await setupTokensDir();
build(tempDir);
cleanUp(tempDir);

function build(cwd: string) {
  const nephrit = new Nephrit({
    source: ['src/tokens/**/*.json'],
    cwd,
    platforms: [
      {
        name: 'web',
        buildPath: 'dist',
        transformGroup: 'web-group',
        files: [
          {
            destination: 'theme.css',
            filter: (t) => {
              console.log({ CHEGOU_TOKEN_AI: t });

              return t.filePath.includes(`/button.tokens/`);
            },
            format: 'css/variables',
          },
        ],
      },
    ],
    logLevel: NephritLogLevel.Trace,
  });

  nephrit.registerFormat({
    name: 'css/variables',
    format: ({ dictionary }) => {
      return dictionary.allTokens
        .map((token) => `--${token.original.path}: ${token.value};`)
        .join('\n');
    },
  });

  nephrit.registerTransform({
    name: 'margin/css/shorthand',
    kind: TransformKind.Value,
    filter: (token) => {
      // check why the actual token value is in snake case
      // @ts-expect-error
      return token.original_value.type === 'margin';
    },
    transform: ({ value }) => {
      const formatMargin = ({
        top,
        right,
        bottom,
        left,
        vertical,
        horizontal,
      }: {
        top?: string;
        right?: string;
        bottom?: string;
        left?: string;
        vertical?: string;
        horizontal?: string;
      }) => {
        if (vertical && horizontal) return `${vertical} ${horizontal}`.trim();

        return `${top} ${right} ${bottom} ${left}`.trim();
      };

      if (Array.isArray(value)) {
        return value.map(formatMargin).join(', ');
      }

      if (typeof value === 'object') {
        return formatMargin(value);
      }

      return value;
    },
  });

  nephrit.registerTransformGroup({
    name: 'web-group',
    transforms: ['margin/css/shorthand'],
  });

  nephrit.registerParser({
    name: 'json',
    pattern: '*.json',
    parser: ({ content }) => {
      return content;
    },
  });

  nephrit.registerAction({
    name: 'hello-word',
    do: () => {
      console.log('Hello, Nephrit!');
    },
    undo: () => {
      console.log('Goodbye, Nephrit!');
    },
  });

  nephrit.buildAll();
}

async function cleanUp(dir: string, prefix = '') {
  const items = await fs.readdir(dir, { withFileTypes: true });
  let output = '';

  for (const item of items) {
    const fullPath = `${dir}/${item.name}`;

    if (item.isDirectory()) {
      output += `${prefix}📁 ${item.name}/\n`;
      output += await cleanUp(fullPath, `${prefix}  `);
    } else {
      output += `${prefix}📄 ${item.name}\n`;
    }
  }

  if (prefix === '') {
    console.log(output.trimEnd());
    await fs.rm(dir, { recursive: true, force: true });
  }

  return output;
}

async function setupTokensDir() {
  const tmpdir = path.join(os.tmpdir(), 'nephrit-test-');
  const folderPath = await fs.mkdtemp(tmpdir);
  await fs.mkdir(path.join(folderPath, 'src', 'tokens'), { recursive: true });
  await fs.cp('example/tokens/', path.join(folderPath, 'src', 'tokens'), {
    recursive: true,
  });

  return folderPath;
}
