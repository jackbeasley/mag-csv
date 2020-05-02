neo4j-admin import --ignore-missing-nodes \
            --nodes Affiliations.csv \
            --nodes Authors.csv \
            --nodes ConferenceInstances.csv \
            --nodes ConferenceSeries.csv \
            --nodes FieldsOfStudy.csv \
            --nodes Journals.csv \
            --nodes Papers.csv \
            --relationships AuthorAffiliations.csv \
            --relationships FieldOfStudyChildren.csv \
            --relationships PaperAuthorAffiliations.csv \
            --relationships PaperConferenceSeries.csv \
            --relationships PaperFieldsOfStudy.csv \
            --relationships PaperReferences.csv



