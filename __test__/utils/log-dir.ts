import fs from 'node:fs/promises';

export async function logDir(dir: string, prefix = '') {
  const items = await fs.readdir(dir, { withFileTypes: true });
  let output = '';

  for (const item of items) {
    const fullPath = `${dir}/${item.name}`;

    if (item.isDirectory()) {
      output += `${prefix}📁 ${item.name}/\n`;
      output += await logDir(fullPath, `${prefix}  `);
    } else {
      output += `${prefix}📄 ${item.name}\n`;
    }
  }

  if (prefix === '') {
    console.log(output.trimEnd());
  }

  return output;
}
