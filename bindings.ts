 // This file has been generated by Specta. DO NOT EDIT.

export type Dataset = { name: string; labels: Label[] }

export type GeneralDataset = { name: string; label_amount: number; data_amount: number }

export type Label = { name: string; data: string[]; is_preprocessed: boolean }

export type ModelDataset = { name: string; data_amount: number }

export type ProgressPayload = { name: string; label: string; current_amount: number; total_amount: number }

export type TauRpcDatasetApiInputTypes = { proc_name: "get_all"; input_type: null } | { proc_name: "get_all_model_dataset"; input_type: null } | { proc_name: "get_random_image"; input_type: { __taurpc_type: string } } | { proc_name: "get_labels"; input_type: { __taurpc_type: string } } | { proc_name: "get_data"; input_type: [string, string] } | { proc_name: "get"; input_type: { __taurpc_type: string } } | { proc_name: "preprocess"; input_type: { __taurpc_type: string } } | { proc_name: "get_image"; input_type: [string, string, string] } | { proc_name: "get_processed_image"; input_type: [string, string, string] }

export type TauRpcDatasetApiOutputTypes = { proc_name: "get_all"; output_type: GeneralDataset[] } | { proc_name: "get_all_model_dataset"; output_type: ModelDataset[] } | { proc_name: "get_random_image"; output_type: string } | { proc_name: "get_labels"; output_type: Label[] } | { proc_name: "get_data"; output_type: string[] } | { proc_name: "get"; output_type: Dataset } | { proc_name: "preprocess"; output_type: null } | { proc_name: "get_image"; output_type: string } | { proc_name: "get_processed_image"; output_type: string | null }

export type TauRpcModelApiInputTypes = { proc_name: "train"; input_type: [string, string, string] }

export type TauRpcModelApiOutputTypes = { proc_name: "train"; output_type: null }

export type TauRpcUtilApiInputTypes = { proc_name: "get_current_dir"; input_type: null } | { proc_name: "open_directory"; input_type: null }

export type TauRpcUtilApiOutputTypes = { proc_name: "get_current_dir"; output_type: string } | { proc_name: "open_directory"; output_type: null }

const ARGS_MAP = {"util":"{\"get_current_dir\":[],\"open_directory\":[]}","model":"{\"train\":[\"dataset_name\",\"model_name\",\"kernel\"]}","dataset":"{\"get_labels\":[\"dataset_name\"],\"get_image\":[\"name\",\"label\",\"data\"],\"get\":[\"dataset_name\"],\"get_data\":[\"dataset_name\",\"label_name\"],\"get_all\":[],\"get_processed_image\":[\"name\",\"label\",\"data\"],\"get_all_model_dataset\":[],\"preprocess\":[\"dataset_name\"],\"get_random_image\":[\"path\"]}"}
import { createTauRPCProxy as createProxy } from "taurpc"

export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

type Router = {
	'dataset': [TauRpcDatasetApiInputTypes, TauRpcDatasetApiOutputTypes],
	'util': [TauRpcUtilApiInputTypes, TauRpcUtilApiOutputTypes],
	'model': [TauRpcModelApiInputTypes, TauRpcModelApiOutputTypes],
}