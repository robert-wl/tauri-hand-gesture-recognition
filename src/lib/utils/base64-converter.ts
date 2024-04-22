export default async function base64Converter(blob: Blob | File): Promise<string> {
  const reader = new FileReader();
  reader.readAsDataURL(blob);

  return await new Promise((res, rej) => {
    reader.onload = () => res(reader.result as string);
    reader.onerror = (error) => rej(error);
  });
}
