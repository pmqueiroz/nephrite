import fs from 'node:fs/promises';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { Nephrit, type NephriteConfig, TransformKind } from '../index';
import { setupTokensDir } from './utils/setup-tokens-dir';

let tempDir: string;

const defaultConfig = () =>
  ({
    source: ['src/tokens/**/*.json'],
    cwd: tempDir,
    platforms: [
      {
        name: 'web',
        buildPath: 'src/dist',
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
  }) satisfies NephriteConfig;

describe('Nephrit', () => {
  beforeEach(async () => {
    tempDir = await setupTokensDir();
  });

  afterEach(async () => {
    await fs.rm(tempDir, { recursive: true, force: true });
  });

  describe('build', () => {
    it('should be a function', () => {
      expect(Nephrit.prototype.buildAll).toBeInstanceOf(Function);
    });

    it('should [wip]', async () => {
      const nephrit = new Nephrit(defaultConfig());

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
            if (vertical && horizontal)
              return `${vertical} ${horizontal}`.trim();

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
    });
  });
});
