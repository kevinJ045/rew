

export type SerializableData = string | number | boolean;
type SerializableDataObject = Record<string, SerializableData>;
export type SerializableObject = Record<string, SerializableData | SerializableDataObject>;

export type BastardObject = Record<string, any>;