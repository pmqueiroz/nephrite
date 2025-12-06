import fs from 'node:fs/promises';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { type Config, Nephrite, TransformKind } from '../index';
import { setupTokensDir } from './utils/setup-tokens-dir';

let tempDir: string;

const defaultConfig = () =>
  ({
    source: ['src/tokens/**/*.json'],
    cwd: tempDir,
  }) satisfies Config;

describe('Nephrite', () => {
  beforeEach(async () => {
    tempDir = await setupTokensDir();
  });

  afterEach(async () => {
    await fs.rm(tempDir, { recursive: true, force: true });
  });

  describe('getConfig', () => {
    it('should be a function', () => {
      expect(Nephrite.prototype.getConfig).toBeInstanceOf(Function);
    });

    it('should return config on getConfig', () => {
      const nephrite = new Nephrite(defaultConfig());

      expect(nephrite.getConfig()).toEqual(
        expect.objectContaining(defaultConfig()),
      );
    });
  });

  describe('build', () => {
    it('should be a function', () => {
      expect(Nephrite.prototype.build).toBeInstanceOf(Function);
    });

    it('should [wip]', async () => {
      const nephrite = new Nephrite(defaultConfig());

      nephrite.registerTransform({
        name: 'margin/css/shorthand',
        kind: TransformKind.Value,
        filter: ({ type }) => type === 'margin',
        transform: ({ value }) => {
          const formatMargin = ({
            top,
            right,
            bottom,
            left,
            vertical,
            horizontal,
          }: any) => {
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

      nephrite.registerTransformGroup({
        name: 'web',
        transforms: ['margin/css/shorthand'],
      });

      nephrite.registerParser({
        name: 'json',
        pattern: '/.json$/',
        transforms: ['margin/css/shorthand'],
      });

      nephrite.registerAction({
        name: 'hello-word',
        do: () => {
          console.log('Hello, Nephrite!');
        },
        undo: () => {
          console.log('Goodbye, Nephrite!');
        },
      });

      nephrite.build();
    });
  });
});
