import type { Product } from "../../../types";
import type {
  NothingResponse,
  Uuid,
  ProductRequest,
} from "./types";
import { HttpClient } from "../../client";
import { APIVersion } from "../../types";

const client = HttpClient.create({ version: APIVersion.V1});

const productService = {
  getProducts(team_id: Uuid) {
    return client.get<Product[]>(`/team/${team_id}/products`);
  },
  createProduct(team_id: Uuid, payload: ProductRequest) {
    return client.post<ProductRequest, Product>(`/team/${team_id}/product`, payload);
  },
  updateProduct(team_id: Uuid, product_id: Uuid, payload: ProductRequest) {
    return client.put<ProductRequest, NothingResponse>(`/team/${team_id}/product/${product_id}`, payload);
  },
  deleteProduct(team_id: Uuid, product_id: Uuid) {
    return client.delete<NothingResponse>(`/team/${team_id}/product/${product_id}`);
  },
};

export default productService;
export { client };
