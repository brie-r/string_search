use std::ops::Range;
pub enum Include { Include, Exclude }
#[derive(Debug)]
pub struct SearchOutput<'a>
{
	pub output: &'a str,
	pub range: Range <usize>,
}
pub trait StringSearch
{
	fn index_of ( &self, range: &Range<usize>, find: &str ) -> Option < Range <usize> >;
	fn index_of_reverse ( &self, range: &Range<usize>, find: &str ) -> Option < Range <usize> >;
	fn index_of_nth ( &self, range: &Range<usize>, find: &str, n: usize ) -> Option < Range <usize> >;
	fn index_of_nth_reverse ( &self, range: &Range<usize>, find: &str, n: usize ) -> Option < Range <usize> >;
	fn index_of_sequence ( &self, range: &Range<usize>, find: &Vec<&str> ) -> Option<Range<usize>>;
	fn index_of_sequence_reverse ( &self, range: &Range<usize>, find: &Vec<&str> ) -> Option<Range<usize>>;
	fn str_search (&self, range: &Range<usize>, start: &Vec<&str>, include_search_str_start: Include, end: &Vec<&str>, include_search_str_end: Include) -> Option<SearchOutput>;
	fn str_search_reverse (&self, range: &Range<usize>, start: &Vec<&str>, include_search_str_start: Include, end: &Vec<&str>, include_search_str_start: Include) -> Option<SearchOutput>;
}
impl StringSearch for str
{
	fn index_of ( &self, index_range: &Range<usize>, find: &str ) -> Option <Range <usize> >
	{
		let substring = &self [ index_range.clone() ];
		match substring.find (find)
		{
			Some ( index_found ) =>
			Some
			(
				index_range.start + index_found ..
				index_range.start + index_found + find.len()
			),
			None => None,
		}
	}
	fn index_of_reverse ( &self, index_range: &Range<usize>, find: &str ) -> Option <Range <usize> >
	{
		let substring = &self [ index_range.clone() ];
		match substring.rfind (find)
		{
			Some ( index_found ) => 
			Some
			(
				index_range.start + index_found ..
				index_range.start + index_found + find.len()
			),
			None => None,
		}
	}
	fn index_of_nth ( &self, index_range: &Range<usize>, find: &str, n: usize) -> Option <Range <usize> >
	{
		let repeat_vec = vec![find; n];
		self.index_of_sequence (index_range, &repeat_vec)
	}
	fn index_of_nth_reverse ( &self, index_range: &Range<usize>, find: &str, n: usize) -> Option <Range <usize> >
	{
		let repeat_vec = vec![find; n];
		self.index_of_sequence_reverse (index_range, &repeat_vec)
	}
	fn index_of_sequence( &self, index_range: &Range<usize>, find: &Vec<&str>) -> Option<Range <usize> >
	{
		let mut output: Option < Range <usize> > = None;
		let mut start = index_range.start;
		for &item in find
		{
			output = match self.index_of( &(start .. index_range.end), &item )
			{
				Some (indices_found) =>
				{
					start = indices_found.end;
					Some ( indices_found )
				},
				None => return None,
			};
		}
		output
	}
	fn index_of_sequence_reverse( &self, index_range: &Range<usize>, find: &Vec<&str>) -> Option<Range <usize> >
	{
		let mut output: Option < Range <usize> > = None;
		let mut start = index_range.end;
		for &item in find
		{
			output = match self.index_of_reverse( &(index_range.start .. start), &item )
			{
				Some (indices_found) =>
				{
					start = indices_found.start;
					Some ( indices_found )
				},
				None => return None,
			};
		}
		output
	}
	fn str_search (&self, index_range: &Range<usize>, start_find: &Vec<&str>, output_pos_start: Include, end_find: &Vec<&str>, output_pos_end: Include) -> Option<SearchOutput>
	{
		let start = match self.index_of_sequence( index_range, start_find )
		{
			Some ( indices_found ) => Some ( indices_found ),
			None => return None,
		}.unwrap(); // unwrap is safe because None returns early
		let end = match self.index_of_sequence( &( start.start .. index_range.end ), end_find)
		{
			Some ( indices_found ) => Some ( indices_found ),
			None => return None,
		}.unwrap(); // unwrap is safe because None returns early

		let start_index = match output_pos_start
		{
			Include::Include => start.start,
			Include::Exclude => start.end,
		};
		let end_index = match output_pos_end
		{
			Include::Include => end.end,
			Include::Exclude => end.start,
		};
		Some
		(
			SearchOutput
			{
				output: &self[ start_index .. end_index ],
				range: start_index .. end_index,
			}
		)
	}
	fn str_search_reverse (&self, index_range: &Range<usize>, end_find: &Vec<&str>, output_pos_end: Include, start_find: &Vec<&str>, output_pos_start: Include) -> Option<SearchOutput>
	{
		let end = match self.index_of_sequence_reverse( index_range, end_find )
		{
			Some ( indices_found ) => Some ( indices_found ),
			None => return None,
		}.unwrap(); // unwrap is safe because None returns early
		let start = match self.index_of_sequence_reverse ( &( index_range.start .. end.start ), start_find)
		{
			Some ( indices_found ) => Some ( indices_found ),
			None => return None,
		}.unwrap(); // unwrap is safe because None returns early

		let end_index = match output_pos_end
		{
			Include::Include => end.end,
			Include::Exclude => end.start,
		};
		let start_index = match output_pos_start
		{
			Include::Include => start.start,
			Include::Exclude => start.end,
		};
		Some
		(
			SearchOutput
			{
				output: &self [ start_index .. end_index],
				range: start_index .. end_index ,
			}
		)
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_index_of()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of( &( 0..text.len() ), "ins");
		let result_range = Some (5..8);
		assert_eq!(result, result_range);
	}
	#[test]
	fn test_index_of_reverse()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of_reverse( &( 0..text.len() ), "ins");
		let result_range = Some (41..44);
		assert_eq!(result, result_range);
	}
	#[test]
	fn test_index_of_nth()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of_nth( &( 0..text.len() ), "p285q", 2);
		let result_range = Some (47..52);
		assert_eq!(result, result_range);
	}
	#[test]
	fn test_index_of_nth_reverse()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of_nth_reverse( &( 0..text.len() ), "p285q", 2);
		let result_range = Some (11..16);
		assert_eq!(result, result_range);
	}
	#[test]
	fn test_none_index_of()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of( &( 0..text.len() ), "text not in string");
		assert!(result.is_none());
	}
	#[test]
	fn test_index_of_sequence()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of_sequence ( &( 0..text.len() ), &vec!["ins", "b3m4x"] );
		let result_range = Some (36..41);
		assert_eq!(result, result_range);
	}
	#[test]
	fn test_index_of_sequence_reverse()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of_sequence_reverse ( &( 0..text.len() ), &vec!["jv7", "gro9"] );
		let result_range = Some (24..28);
		assert_eq!(result, result_range);
	}
	#[test]
	fn test_none_index_of_sequence()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.index_of_sequence ( &( 0..text.len() ), &vec!["text not in string", "b3m4x"] );
		assert!(result.is_none());
	}
	#[test]
	fn test_str_search()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.str_search ( &( 0..text.len() ), &vec!["ins", "b3m4x"], Include::Exclude, &vec!["q0t", "jv7"], Include::Exclude ).unwrap();
		let result_range = 41 .. 57;
		assert_eq!(result.range, result_range);
		assert_eq!(result.output, "ins1ekp285q0tzdl");
	}
	#[test]
	fn test_str_search_include()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.str_search ( &( 0..text.len() ), &vec!["ins", "b3m4x"], Include::Include, &vec!["q0t", "jv7"], Include::Include ).unwrap();
		let result_range = 36 .. 60;
		assert_eq!(result.range, result_range);
		assert_eq!(result.output, "b3m4xins1ekp285q0tzdljv7");
	}
	#[test]
	fn test_str_search_reverse()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.str_search_reverse ( &( 0..text.len() ), &vec!["jv7", "gro9"], Include::Exclude, &vec!["ekp2", "4xin"], Include::Exclude ).unwrap();
		let result_range = 7 .. 24;
		assert_eq!(result.range, result_range);
		assert_eq!(result.output, "s1ekp285q0tzdljv7");
	}
	#[test]
	fn test_str_search_reverse_include()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.str_search_reverse ( &( 0..text.len() ), &vec!["jv7", "gro9"], Include::Include, &vec!["ekp2", "4xin"], Include::Include ).unwrap();
		let result_range = 3 .. 28;
		assert_eq!(result.range, result_range);
		assert_eq!(result.output, "4xins1ekp285q0tzdljv7gro9");
	}
	#[test]
	fn test_none_str_search()
	{
		let text = "b3m4xins1ekp285q0tzdljv7gro9hcwfuay6b3m4xins1ekp285q0tzdljv7gro9hcwfuay6";
		let result = text.str_search ( &( 0..text.len() ), &vec!["text not in string", "b3m4x"], Include::Exclude, &vec!["q0t", "jv7"], Include::Exclude );
		assert!(result.is_none());
	}
}
