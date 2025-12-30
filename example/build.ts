import fs from 'node:fs/promises';
import { Nephrit, NephritLogLevel, TransformKind } from '../index.js';

build();
logStructure('example/dist');

function build() {
  const nephrit = new Nephrit({
    source: ['tokens/**/*.json'],
    cwd: 'example/',
    platforms: [
      {
        name: 'web',
        buildPath: 'dist',
        transformGroup: 'web-group',
        files: [
          {
            destination: 'theme.css',
            filter: (t) => t.filePath.includes(`/button.tokens/`),
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
      const tokens = dictionary.allTokens
        .map((token) => `\t${token.name}: ${token.value};`)
        .join('\n');

      return `.design-system {\n${tokens}\n}`;
    },
  });

  nephrit.registerTransform({
    name: 'css/variables',
    kind: TransformKind.Name,
    filter: () => true,
    transform: ({ name }) => {
      return `--${name}`;
    },
  });

  nephrit.registerTransform({
    name: 'margin/css/shorthand',
    kind: TransformKind.Value,
    filter: (token) => {
      return token.original.type === 'margin';
    },
    transform: ({ original: { value } }) => {
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
    transforms: ['margin/css/shorthand', 'css/variables'],
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

async function logStructure(dir: string, prefix = '') {
  const items = await fs.readdir(dir, { withFileTypes: true });
  let output = '';

  for (const item of items) {
    const fullPath = `${dir}/${item.name}`;

    if (item.isDirectory()) {
      output += `${prefix}📁 ${item.name}/\n`;
      output += await logStructure(fullPath, `${prefix}  `);
    } else {
      output += `${prefix}📄 ${item.name}\n`;
    }
  }

  if (prefix === '') {
    console.log(output.trimEnd());
  }

  return output;
}
