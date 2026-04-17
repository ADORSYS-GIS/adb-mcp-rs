pub struct DeviceInfo {
    pub serial: String,
    pub state: String,
    pub model: Option<String>,
}

pub struct ElementBounds {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

pub fn parse_devices(output: &str) -> Vec<DeviceInfo> {
    output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                Some(DeviceInfo {
                    serial: parts[0].to_string(),
                    state: parts[1].to_string(),
                    model: parts
                        .get(2)
                        .and_then(|s| s.strip_prefix("model:").map(|m| m.to_string())),
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn extract_bounds(line: &str) -> Option<ElementBounds> {
    let start = line.find("bounds=\"")?;
    let rest = &line[start + 8..];
    let end = rest.find("\"")?;
    let bounds_str = &rest[..end];

    let cleaned: String = bounds_str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',')
        .collect();

    let parts: Vec<u32> = cleaned.split(',').filter_map(|s| s.parse().ok()).collect();

    if parts.len() >= 4 {
        Some(ElementBounds {
            left: parts[0],
            top: parts[1],
            right: parts[2],
            bottom: parts[3],
        })
    } else {
        None
    }
}

pub fn find_element_in_xml(
    xml: &str,
    text: Option<&str>,
    resource_id: Option<&str>,
    content_desc: Option<&str>,
    index: usize,
) -> Option<(u32, u32)> {
    let mut current_index = 0;

    for line in xml.lines() {
        let matches_text = text.map_or(true, |t| line.contains(&format!("text=\"{}\"", t)));
        let matches_id =
            resource_id.map_or(true, |id| line.contains(&format!("resource-id=\"{}\"", id)));
        let matches_desc =
            content_desc.map_or(true, |d| line.contains(&format!("content-desc=\"{}\"", d)));

        if matches_text && matches_id && matches_desc {
            if current_index == index {
                let bounds = extract_bounds(line)?;
                let center_x = (bounds.left + bounds.right) / 2;
                let center_y = (bounds.top + bounds.bottom) / 2;
                return Some((center_x, center_y));
            }
            current_index += 1;
        }
    }

    None
}

pub fn parse_ip_address(output: &str) -> Option<String> {
    let ip = output.trim();
    if ip.contains('.') && !ip.is_empty() {
        Some(ip.to_string())
    } else {
        None
    }
}
