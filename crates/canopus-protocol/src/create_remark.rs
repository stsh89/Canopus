use crate::Result;
use std::future::Future;
use uuid::Uuid;

pub struct NewRemark {
    pub essence: String,
    pub tags: Vec<String>,
}

pub trait SaveRemark {
    fn save_remark(&self, new_remark: NewRemark) -> impl Future<Output = Result<Uuid>>;
}

pub async fn create_remark(new_remark: NewRemark, repository: &impl SaveRemark) -> Result<Uuid> {
    let NewRemark { essence, tags } = new_remark;

    let new_remark = NewRemark {
        essence: sanitize_essence(essence),
        tags: tags.into_iter().map(sanitize_tag).collect(),
    };

    repository.save_remark(new_remark).await
}

fn sanitize_essence(essence: String) -> String {
    essence.trim().to_string()
}

fn sanitize_tag(tag: String) -> String {
    tag.trim().to_string()
}
