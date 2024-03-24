use async_trait::async_trait;

#[async_trait]
trait FileStorage {
    async fn upload(&self, file_path: str);

    async fn delete(&self);

}
