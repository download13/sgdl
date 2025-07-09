#[derive(Debug, Clone)]
enum ProviderDomain {
	Kemono,
	Coomer,
}

impl ProviderDomain {
	pub fn from_str(endpoint: &str) -> Option<Self> {
		match endpoint {
			"kemono.su" => Some(Self::Kemono),
			"coomer.su" => Some(Self::Coomer),
			_ => None,
		}
	}

	pub fn as_str(&self) -> &str {
		match self {
			Self::Kemono => "kemono.su",
			Self::Coomer => "coomer.su",
		}
	}
}

#[derive(Debug, Clone)]
pub struct PostPointer {
	pub creator: ProfilePointer,
	pub post_id: String,
}

impl PostPointer {
	pub fn from_url(url: &str) -> Option<Self> {
		if let Some(captures) = POST_URL_RE.captures(url) {
			let provider = ProviderDomain::from_str(captures.get(1).unwrap().as_str())?;

			let service_slug = captures.get(2).unwrap().as_str();
			let creator_id = captures.get(3).unwrap().as_str();
			let post_id = captures.get(4).unwrap().as_str();

			return Some(Self {
				creator: ProfilePointer {
					provider_domain: provider,
					service_slug: service_slug.to_string(),
					creator_id: creator_id.to_string(),
				},
				post_id: post_id.to_string(),
			});
		}

		None
	}

	pub fn to_url(&self) -> String {
		format!("{}/post/{}", self.creator.to_url(), self.post_id).to_string()
	}
}

#[derive(Debug, Clone)]
pub struct ProfilePointer {
	pub provider_domain: ProviderDomain,
	pub service_slug: String,
	pub creator_id: String,
}

impl ProfilePointer {
	pub fn from_url(url: &str) -> Option<Self> {
		if let Some(captures) = PROFILE_URL_RE.captures(url) {
			let provider = ProviderDomain::from_str(captures.get(1).unwrap().as_str())?;

			let service_slug = captures.get(2).unwrap().as_str();
			let creator_id = captures.get(3).unwrap().as_str();

			return Some(Self {
				provider_domain: provider,
				service_slug: service_slug.to_string(),
				creator_id: creator_id.to_string(),
			});
		}

		None
	}

	pub fn to_url(&self) -> String {
		format!(
			"https://{}/{}/user/{}",
			self.provider_domain.as_str(),
			self.service_slug,
			self.creator_id
		)
	}
}

lazy_static::lazy_static! {
	static ref PROFILE_URL_RE: regex::Regex =
		regex::Regex::new(r"https://(kemono\.su|coomer\.su)/([^/]+?)/user/([^/]+?)/?").unwrap();
		static ref POST_URL_RE: regex::Regex =
		regex::Regex::new(r"https://(kemono\.su|coomer\.su)/([^/]+?)/user/([^/]+?)/post/([^/]+?)/?").unwrap();
}
